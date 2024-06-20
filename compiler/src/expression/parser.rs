use serde::{Serialize, Serializer};
use serde::ser::SerializeSeq;
use crate::expression::lexer::{ExpressionLex, ExpressionLexer};
use crate::expression::token::{ExpressionAtom, ExpressionOperator, ExpressionToken};
use crate::{Attribution, Problem};
use crate::symbol::{SymbolList};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExpressionParse {
    Atom(ExpressionAtom),
    Operation(ExpressionOperator, Vec<ExpressionParse>),
}

impl Serialize for ExpressionParse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            ExpressionParse::Atom(atom) => atom.serialize(serializer),
            ExpressionParse::Operation(operator, operands) => {
                let mut sequence = serializer.serialize_seq(Some(operands.len() + 1))?;
                sequence.serialize_element(operator)?;
                for operand in operands {
                    sequence.serialize_element(operand)?;
                }
                sequence.end()
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TagParse {
    When(ExpressionParse),
    Else(Option<ExpressionParse>),
    End,
}

pub struct ExpressionParsingResult {
    pub parse: Option<ExpressionParse>,
    pub problems: Vec<Problem>,
}

pub struct TagParsingResult {
    pub parse: Option<TagParse>,
    pub problems: Vec<Problem>,
}

pub struct ExpressionParser<'a> {
    lexer: ExpressionLexer<'a>,
}

struct Parse<'a> {
    problems: &'a mut Vec<Problem>,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(symbols: &'a SymbolList) -> Self {
        Self { lexer: ExpressionLexer::new(symbols) }
    }

    pub fn parse_expression(&self, lex: &mut ExpressionLex, attribution: &Attribution) -> ExpressionParsingResult {
        let mut problems = Vec::new();
        let parse = Parse { problems: &mut problems }.parse_right(lex, attribution, ExpressionOperator::OpenParen);
        ExpressionParsingResult { parse, problems }
    }

    pub fn parse_tag(&self, lex: &mut ExpressionLex, attribution: &Attribution) -> TagParsingResult {
        let result = match lex.peek().map(|marked| marked.token) {
            None => TagParsingResult { parse: None, problems: Vec::new() },
            Some(ExpressionToken::Operator(ExpressionOperator::End)) => TagParsingResult { parse: Some(TagParse::End), problems: Vec::new() },
            Some(ExpressionToken::Operator(ExpressionOperator::Else)) => {
                lex.next();
                let result = self.parse_expression(lex, attribution);
                TagParsingResult { parse: Some(TagParse::Else(result.parse)), problems: result.problems }
            },
            _ => {
                let result = self.parse_expression(lex, attribution);
                TagParsingResult { parse: result.parse.map(|e| TagParse::When(e)), problems: result.problems }
            },
        };
        result
    }

    pub fn parse(&self, source: &str, attribution: &Attribution) -> ExpressionParsingResult {
        self.parse_expression(&mut self.lexer.lex(source), attribution)
    }
}

impl<'a> Parse<'a> {
    fn parse_right(&mut self, lex: &mut ExpressionLex, attribution: &Attribution, operator: ExpressionOperator) -> Option<ExpressionParse> {
        let left = if let Some(left_token) = lex.next() {
            let left_attribution = attribution.at_marks(left_token.start_mark, left_token.end_mark);
            match left_token.token {
                ExpressionToken::Atom(atom) => Some(ExpressionParse::Atom(atom)),
                ExpressionToken::Operator(prefix_operator) => {
                    match prefix_operator {
                        ExpressionOperator::OpenParen => {
                            if let Some(left) = self.parse_right(lex, attribution, ExpressionOperator::OpenParen) {
                                if let None = lex.match_operator(ExpressionOperator::CloseParen) {
                                    self.problems.push(Problem::fatal("Expected a closing paren", &left_attribution));
                                }
                                Some(left)
                            } else {
                                None
                            }
                        },
                        ExpressionOperator::Of | ExpressionOperator::From | ExpressionOperator::Comma => {
                            return self.parse_right(lex, attribution, operator)
                        },
                        ExpressionOperator::When => {
                            self.parse_right(lex, attribution, prefix_operator)
                        },
                        ExpressionOperator::Not | ExpressionOperator::Unless => {
                            if let Some(left) = self.parse_right(lex, attribution, prefix_operator) {
                                Some(ExpressionParse::Operation(ExpressionOperator::Not, vec!(left)))
                            } else {
                                None
                            }
                        },
                        ExpressionOperator::In => {
                            if let Some(ExpressionParse::Atom(ExpressionAtom::Reference(name))) = self.parse_right(lex, attribution, prefix_operator) {
                                Some(ExpressionParse::Operation(ExpressionOperator::In, vec!(ExpressionParse::Atom(ExpressionAtom::Reference(name)))))
                            } else {
                                self.problems.push(Problem::fatal("Expected a reference to a location", attribution));
                                None
                            }
                        }
                        ExpressionOperator::Either => {
                            if let Some(left) = self.parse_right(lex, attribution, prefix_operator) {
                                match left {
                                    ExpressionParse::Atom(atom) => Some(ExpressionParse::Atom(atom)),
                                    ExpressionParse::Operation(operator, operands) => {
                                        match operator {
                                            ExpressionOperator::And | ExpressionOperator::Or => Some(ExpressionParse::Operation(ExpressionOperator::Either, operands)),
                                            _ => Some(ExpressionParse::Operation(operator, operands)),
                                        }
                                    }
                                }
                            } else {
                                None
                            }
                        },
                        ExpressionOperator::Maximum | ExpressionOperator::Minimum => {
                            if let Some(left) = self.parse_right(lex, attribution, prefix_operator) {
                                match left {
                                    ExpressionParse::Atom(atom) => Some(ExpressionParse::Atom(atom)),
                                    ExpressionParse::Operation(operator, operands) => {
                                        match operator {
                                            ExpressionOperator::And | ExpressionOperator::Or | ExpressionOperator::Either | ExpressionOperator::Between => {
                                                Some(ExpressionParse::Operation(prefix_operator, operands))
                                            },
                                            _ => Some(ExpressionParse::Operation(operator, operands))
                                        }
                                    },
                                }
                            } else {
                                None
                            }
                        }
                        ExpressionOperator::Random => {
                            if let Some(left) = self.parse_right(lex, attribution, prefix_operator) {
                                match left {
                                    ExpressionParse::Atom(atom) => Some(ExpressionParse::Atom(atom)),
                                    ExpressionParse::Operation(operator, operands) => {
                                        match operator {
                                            ExpressionOperator::And | ExpressionOperator:: Or => Some(ExpressionParse::Operation(ExpressionOperator::Either, operands)),
                                            _ => Some(ExpressionParse::Operation(operator, operands)),
                                        }
                                    }
                                }
                            } else {
                                None
                            }
                        },
                        ExpressionOperator::Between => {
                            if let Some(left) = self.parse_right(lex, attribution, prefix_operator) {
                                match left {
                                    ExpressionParse::Atom(atom) => Some(ExpressionParse::Atom(atom)),
                                    ExpressionParse::Operation(operator, operands) => {
                                        match operator {
                                            ExpressionOperator::And | ExpressionOperator:: Or => Some(ExpressionParse::Operation(prefix_operator, operands)),
                                            _ => Some(ExpressionParse::Operation(operator, operands)),
                                        }
                                    }
                                }
                            } else {
                                None
                            }
                        },
                        _ => {
                            self.problems.push(Problem::fatal("Not a valid prefix operator", &left_attribution));
                            self.parse_right(lex, attribution, operator)
                        },
                    }
                },
                ExpressionToken::UnrecognizedToken => {
                    self.problems.push(Problem::fatal("Unrecognized token", &left_attribution));
                    self.parse_right(lex, attribution, operator)
                },
            }
        } else {
            None
        };

        if let Some(mut left) = left {
            while let Some(right_marked_token) = lex.next_operator(operator) {
                let right_token = right_marked_token.token;
                let right_attribution = attribution.at_marks(right_marked_token.start_mark, right_marked_token.end_mark);
                if let ExpressionToken::Operator(right_operator) = right_token {
                    left = match right_operator {
                        ExpressionOperator::Equal |
                        ExpressionOperator::NotEqual |
                        ExpressionOperator::GreaterThan |
                        ExpressionOperator::GreaterThanOrEqual |
                        ExpressionOperator::LessThan |
                        ExpressionOperator::LessThanOrEqual => {
                            if let Some(right) = self.parse_right(lex, attribution, right_operator) {
                                ExpressionParse::Operation(right_operator, vec!(left, right))
                            } else {
                                break;
                            }
                        },

                        ExpressionOperator::Plus |
                        ExpressionOperator::Minus |
                        ExpressionOperator::Multiply |
                        ExpressionOperator::Divide |
                        ExpressionOperator::And |
                        ExpressionOperator::Or => {
                            if let Some(right) = self.parse_right(lex, attribution, right_operator) {
                                match right {
                                    ExpressionParse::Atom(atom) => ExpressionParse::Operation(right_operator, vec!(left, ExpressionParse::Atom(atom))),
                                    ExpressionParse::Operation(operator, mut operands) => {
                                        if operator == right_operator {
                                            operands.insert(0, left);
                                            ExpressionParse::Operation(right_operator, operands)
                                        } else {
                                            ExpressionParse::Operation(right_operator, vec!(left, ExpressionParse::Operation(operator, operands)))
                                        }
                                    },
                                }
                            } else {
                                break;
                            }
                        },

                        ExpressionOperator::Comma => {
                            if let Some(next) = lex.peek() {
                                match next.token {
                                    ExpressionToken::Atom(..) => {
                                        if let Some(right) = self.parse_right(lex, attribution, right_operator) {
                                            if let ExpressionParse::Operation(operator, mut operands) = right {
                                                operands.insert(0, left);
                                                ExpressionParse::Operation(operator, operands)
                                            } else {
                                                ExpressionParse::Operation(ExpressionOperator::And, vec!(left, right))
                                            }
                                        } else {
                                            break;
                                        }
                                    },
                                    _ => {
                                        continue;
                                    },
                                }
                            } else {
                                break;
                            }
                        },

                        ExpressionOperator::To => {
                            if let Some(right) = self.parse_right(lex, attribution, right_operator) {
                                ExpressionParse::Operation(ExpressionOperator::Between, vec!(left, right))
                            } else {
                                break;
                            }
                        },

                        ExpressionOperator::Then => {
                            if let Some(middle) = self.parse_right(lex, attribution, ExpressionOperator::Then) {
                                lex.match_operator(ExpressionOperator::Comma);
                                if let None = lex.match_operator(ExpressionOperator::Else) {
                                    self.problems.push(Problem::fatal("Expected `else` here", &right_attribution));
                                }
                                lex.match_operator(ExpressionOperator::Comma);
                                if let Some(right) = self.parse_right(lex, attribution, ExpressionOperator::Then) {
                                    ExpressionParse::Operation(ExpressionOperator::Then, vec!(left, middle, right))
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        _ => {
                            self.problems.push(Problem::fatal("Not a valid infix operator", &right_attribution));
                            break;
                        }
                    }
                } else {
                    panic!();
                }
            }

            Some(left)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub fn normalize_expression(expression: &ExpressionParse) -> ExpressionParse {
    return match expression {
        ExpressionParse::Atom(atom) => ExpressionParse::Atom(atom.clone()),
        ExpressionParse::Operation(operator, operands) => {
            match operator {
                ExpressionOperator::Not => {
                    let operand = normalize_expression(&operands[0]);
                    match operand {
                        ExpressionParse::Atom(atom) =>
                            match atom {
                                ExpressionAtom::NumericLiteral(n) => ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(n == 0)),
                                ExpressionAtom::LogicalLiteral(b) => ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(!b)),
                                _ => ExpressionParse::Operation(ExpressionOperator::Not, vec!(ExpressionParse::Atom(atom))),
                            },
                        ExpressionParse::Operation(operator, operands) =>
                            ExpressionParse::Operation(ExpressionOperator::Not, vec!(ExpressionParse::Operation(operator, operands))),
                    }
                },
                ExpressionOperator::And => {
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(&operand);
                        match &operand {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(n)) => {
                                if *n > 0 {
                                    continue;
                                } else {
                                    return ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(false));
                                }
                            },
                            ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(b)) => {
                                if *b {
                                    continue;
                                } else {
                                    return ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(false))
                                }
                            },
                            _ => result.push(operand),
                        };
                    }
                    ExpressionParse::Operation(ExpressionOperator::And, result)
                },
                ExpressionOperator::Or => {
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(&operand);
                        match &operand {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(n)) => {
                                if *n <= 0 {
                                    continue;
                                } else {
                                    return ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(true));
                                }
                            },
                            ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(b)) => {
                                if !b {
                                    continue;
                                } else {
                                    return ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(true))
                                }
                            },
                            _ => result.push(operand),
                        };
                    }
                    ExpressionParse::Operation(ExpressionOperator::Or, result)
                },
                ExpressionOperator::Plus => {
                    let mut constant = 0u32;
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(&operand);
                        match &operand {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(n)) => {
                                constant += n;
                            },
                            ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(b)) => {
                                if *b {
                                    constant += 1;
                                }
                            },
                            _ => result.push(operand),
                        };
                    }
                    if constant > 0 {
                        result.push(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0)));
                    }
                    if result.is_empty() {
                        ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0))
                    } else {
                        ExpressionParse::Operation(ExpressionOperator::Plus, result)
                    }
                },
                ExpressionOperator::Multiply => {
                    let mut constant = 1u32;
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(&operand);
                        match &operand {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(n)) => {
                                if *n == 0 {
                                    return ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0));
                                }
                                constant *= n;
                            },
                            ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(b)) => {
                                if !b {
                                    return ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0));
                                }
                            },
                            _ => result.push(operand),
                        };
                    }
                    if constant > 1 {
                        result.push(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0)));
                    }
                    if result.is_empty() {
                        ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1))
                    } else {
                        ExpressionParse::Operation(ExpressionOperator::Multiply, result)
                    }
                },
                ExpressionOperator::Minus => {
                    let mut constant = None;
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(&operand);
                        match &operand {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(n)) => {
                                if let Some(m) = constant {
                                    constant = Some(m - n);
                                } else {
                                    constant = Some(*n);
                                }
                            },
                            ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(b)) => {
                                if *b {
                                    if let Some(m) = constant {
                                        constant = Some(m - 1);
                                    }
                                } else {
                                    if let None = constant {
                                        constant = Some(0);
                                    }
                                }
                            },
                            _ => {
                                if let Some(m) = constant {
                                    result.push(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(m)));
                                    constant = None;
                                }
                                result.push(operand);
                            },
                        };
                    }
                    if let Some(m) = constant {
                        if result.is_empty() {
                            return ExpressionParse::Atom(ExpressionAtom::NumericLiteral(m));
                        } else {
                            result.push(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(m)));
                        }
                    }
                    ExpressionParse::Operation(ExpressionOperator::Minus, result)
                },
                ExpressionOperator::Divide => {
                    let mut constant = None;
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(&operand);
                        match &operand {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(n)) => {
                                if *n == 0 {
                                    return ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0));
                                }

                                if let Some(m) = constant {
                                    constant = Some(m / n);
                                } else {
                                    constant = Some(*n);
                                }
                            },
                            ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(b)) => {
                                if !b {
                                    return ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0));
                                }

                                if let None = constant {
                                    constant = Some(1);
                                }
                            },
                            _ => {
                                if let Some(m) = constant {
                                    result.push(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(m)));
                                    constant = None;
                                }
                                result.push(operand);
                            },
                        };
                    }
                    if let Some(m) = constant {
                        if result.is_empty() {
                            return ExpressionParse::Atom(ExpressionAtom::NumericLiteral(m));
                        } else {
                            result.push(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(m)));
                        }
                    }
                    ExpressionParse::Operation(ExpressionOperator::Divide, result)
                },
                ExpressionOperator::Maximum => {
                    let mut constant = 0u32;
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(operand);
                        match operand {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(n)) => {
                                if n > constant {
                                    constant = n;
                                }
                            },
                            ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(b)) => {
                                if b && constant < 1 {
                                    constant = 1;
                                }
                            },
                            _ => {
                                result.push(operand);
                            }
                        }
                    }
                    if result.is_empty() {
                        ExpressionParse::Atom(ExpressionAtom::NumericLiteral(constant))
                    } else {
                        if constant > 0 {
                            result.push(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(constant)));
                        }
                        ExpressionParse::Operation(ExpressionOperator::Maximum, result)
                    }
                },
                ExpressionOperator::Minimum => {
                    let mut constant = None;
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(operand);
                        match operand {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(n)) => {
                                if n == 0 {
                                    return ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0));
                                }

                                if let Some(m) = constant {
                                    if n < m {
                                        constant = Some(n);
                                    }
                                } else {
                                    constant = Some(n);
                                }
                            },
                            ExpressionParse::Atom(ExpressionAtom::LogicalLiteral(b)) => {
                                if !b {
                                    return ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0));
                                }

                                if let Some(m) = constant {
                                    if m > 1 {
                                        constant = Some(1);
                                    }
                                } else {
                                    constant = Some(1);
                                }
                            },
                            _ => {
                                result.push(operand);
                            }
                        }
                    }
                    if result.is_empty() {
                        if let Some(m) = constant {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(m))
                        } else {
                            ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0))
                        }
                    } else {
                        if let Some(m) = constant {
                            result.push(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(m)));
                        }
                        ExpressionParse::Operation(ExpressionOperator::Maximum, result)
                    }
                },
                _ => {
                    let mut result = Vec::new();
                    for operand in operands {
                        let operand = normalize_expression(operand);
                        result.push(operand);
                    }
                    ExpressionParse::Operation(*operator, result)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Mark;
    use super::*;

    #[test]
    pub fn test_parser() {
        let mut symbols = SymbolList::new();
        symbols.push("a river in time");
        symbols.push("a river in space");
        let parser = ExpressionParser::new(&symbols);

        let attribution = Attribution::new("test", Mark { line: 0, column: 0 }, Mark { line: 0, column: 0 });

        let result = parser.parse("1 + 2", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(ExpressionOperator::Plus, vec!(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1)), ExpressionParse::Atom(ExpressionAtom::NumericLiteral(2))))));
        assert!(result.problems.is_empty());

        let result = parser.parse("1 + 2 * 4", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Plus,
            vec!(
                ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1)),
                ExpressionParse::Operation(
                    ExpressionOperator::Multiply,
                    vec!(
                        ExpressionParse::Atom(ExpressionAtom::NumericLiteral(2)),
                        ExpressionParse::Atom(ExpressionAtom::NumericLiteral(4)),
                    ),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("1 * 2 + 4", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Plus,
            vec!(
                ExpressionParse::Operation(
                    ExpressionOperator::Multiply,
                    vec!(
                        ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1)),
                        ExpressionParse::Atom(ExpressionAtom::NumericLiteral(2)),
                    ),
                ),
                ExpressionParse::Atom(ExpressionAtom::NumericLiteral(4)),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("a river in time > 7", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::GreaterThan,
            vec!(
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in time")),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(7),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("between a river in space and a river in time", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Between,
            vec!(
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in space")),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in time")),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("one of 1, 2, or no", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Either,
            vec!(
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(1),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(2),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::LogicalLiteral(false),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("maximum of a river in space or a river in time", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Maximum,
            vec!(
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in space")),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in time")),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("(1 + 2) * 4", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Multiply,
            vec!(
                ExpressionParse::Operation(
                    ExpressionOperator::Plus,
                    vec!(
                        ExpressionParse::Atom(
                            ExpressionAtom::NumericLiteral(1),
                        ),
                        ExpressionParse::Atom(
                            ExpressionAtom::NumericLiteral(2),
                        ),
                    )
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(4),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("a river in space then 4 else 5", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Then,
            vec!(
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in space")),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(4),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(5),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("a river in space then a river in time then 4 else 5 else 6", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Then,
            vec!(
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in space")),
                ),
                ExpressionParse::Operation(
                    ExpressionOperator::Then,
                    vec!(
                        ExpressionParse::Atom(
                            ExpressionAtom::Reference(String::from("a river in time")),
                        ),
                        ExpressionParse::Atom(
                            ExpressionAtom::NumericLiteral(4),
                        ),
                        ExpressionParse::Atom(
                            ExpressionAtom::NumericLiteral(5),
                        ),
                    ),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(6),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("a river in space, then, 4 otherwise a river in time, then 5, else 6", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Then,
            vec!(
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in space")),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(4),
                ),
                ExpressionParse::Operation(
                    ExpressionOperator::Then,
                    vec!(
                        ExpressionParse::Atom(
                            ExpressionAtom::Reference(String::from("a river in time")),
                        ),
                        ExpressionParse::Atom(
                            ExpressionAtom::NumericLiteral(5),
                        ),
                        ExpressionParse::Atom(
                            ExpressionAtom::NumericLiteral(6),
                        ),
                    ),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("not 1 or 2 and 3", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::And,
            vec!(
                ExpressionParse::Operation(
                    ExpressionOperator::Or,
                    vec!(
                        ExpressionParse::Operation(
                            ExpressionOperator::Not,
                            vec!(
                                ExpressionParse::Atom(
                                    ExpressionAtom::NumericLiteral(1),
                                ),
                            ),
                        ),
                        ExpressionParse::Atom(
                            ExpressionAtom::NumericLiteral(2),
                        ),
                    ),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::NumericLiteral(3),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("unless 1 or 2 and 3", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Not,
            vec!(
                ExpressionParse::Operation(
                    ExpressionOperator::And,
                    vec!(
                        ExpressionParse::Operation(
                            ExpressionOperator::Or,
                            vec!(
                                ExpressionParse::Atom(
                                    ExpressionAtom::NumericLiteral(1),
                                ),
                                ExpressionParse::Atom(
                                    ExpressionAtom::NumericLiteral(2),
                                ),
                            ),
                        ),
                        ExpressionParse::Atom(
                            ExpressionAtom::NumericLiteral(3),
                        ),
                    ),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let result = parser.parse("in a river in space or a river in time", &attribution);
        assert_eq!(result.parse, Some(ExpressionParse::Operation(
            ExpressionOperator::Or,
            vec!(
                ExpressionParse::Operation(
                    ExpressionOperator::In,
                    vec!(
                        ExpressionParse::Atom(
                            ExpressionAtom::Reference(String::from("a river in space")),
                        ),
                    ),
                ),
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in time")),
                ),
            ),
        )));
        assert!(result.problems.is_empty());

        let lexer = ExpressionLexer::new(&symbols);
        let result = parser.parse_tag(&mut lexer.lex("unless a river in space"), &attribution);
        assert_eq!(result.parse, Some(TagParse::When(ExpressionParse::Operation(
            ExpressionOperator::Not,
            vec!(
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in space")),
                ),
            ),
        ))));
        assert!(result.problems.is_empty());

        let result = parser.parse_tag(&mut lexer.lex("else a river in time"), &attribution);
        assert_eq!(result.parse, Some(TagParse::Else(Some(ExpressionParse::Atom(
            ExpressionAtom::Reference(String::from("a river in time")),
        )))));
        assert!(result.problems.is_empty());

        let result = parser.parse_tag(&mut lexer.lex("else"), &attribution);
        assert_eq!(result.parse, Some(TagParse::Else(None)));
        assert!(result.problems.is_empty());

        let result = parser.parse_tag(&mut lexer.lex("end"), &attribution);
        assert_eq!(result.parse, Some(TagParse::End));
        assert!(result.problems.is_empty());
    }
}
