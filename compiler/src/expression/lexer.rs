use crate::expression::{MarkedToken};
use crate::expression::token::{ExpressionAtom, ExpressionOperator, ExpressionToken};
use crate::Mark;
use crate::symbol::{SymbolList};

pub struct ExpressionLexer<'a> {
    symbols: &'a SymbolList,
}

pub struct ExpressionLex<'a> {
    lexer: &'a ExpressionLexer<'a>,
    source: &'a str,
    line: usize,
    column: usize,
}

impl<'a> ExpressionLexer<'a> {
    pub fn new(symbols: &'a SymbolList) -> Self {
        Self {
            symbols,
        }
    }

    pub fn lex(&'a self, source: &'a str) -> ExpressionLex<'a> {
        ExpressionLex {
            lexer: self,
            source,
            line: 0,
            column: 0,
        }
    }

    pub fn lex_at_mark(&'a self, source: &'a str, mark: Mark) -> ExpressionLex<'a> {
        ExpressionLex {
            lexer: self,
            source,
            line: mark.line as usize,
            column: mark.column as usize,
        }
    }
}

impl<'a> ExpressionLex<'a> {
    pub fn mark(&self) -> Mark {
        Mark {
            line: self.line as u64,
            column: self.column as u64,
        }
    }

    pub fn match_operator(&mut self, operator: ExpressionOperator) -> Option<MarkedToken<ExpressionToken>> {
        let (offset, token, after_line, after_column) = self.advance();

        if let Some(ExpressionToken::Operator(o)) = token {
            if o == operator {
                let start_mark = self.mark();
                self.source = &self.source[offset..];
                self.line = after_line;
                self.column = after_column;
                let end_mark = self.mark();
                return Some(MarkedToken {
                    start_mark,
                    end_mark,
                    token: ExpressionToken::Operator(o),
                });
            }
        }

        None
    }

    pub fn peek(&self) -> Option<MarkedToken<ExpressionToken>> {
        let (_, token, line, column) = self.advance();
        token.map(|token| {
            let start_mark = self.mark();
            let end_mark = Mark { line: line as u64, column: column as u64 };
            MarkedToken {
                start_mark,
                end_mark,
                token,
            }
        })
    }

    pub fn next_operator(&mut self, outer: ExpressionOperator) -> Option<MarkedToken<ExpressionToken>> {
        let (offset, token, after_line, after_column) = self.advance();
        if let Some(ExpressionToken::Operator(operator)) = token {
            if operator >= outer {
                let start_mark = self.mark();
                self.source = &self.source[offset..];
                self.line = after_line;
                self.column = after_column;
                let end_mark = self.mark();
                return Some(MarkedToken {
                    start_mark,
                    end_mark,
                    token: ExpressionToken::Operator(operator),
                });
            }
        }

        None
    }

    pub fn next(&mut self) -> Option<MarkedToken<ExpressionToken>> {
        let (offset, token, line, column) = self.advance();
        let start_mark = self.mark();
        self.source = &self.source[offset..];
        self.line = line;
        self.column = column;
        token.map(|token| {
            let end_mark = self.mark();
            MarkedToken {
                start_mark,
                end_mark,
                token,
            }
        })
    }

    fn advance(&self) -> (usize, Option<ExpressionToken>, usize, usize) {
        let mut offset = 0usize;
        let mut line = self.line;
        let mut column = self.column;

        let mut chars = self.source.chars();
        while let Some(char) = chars.next() {
            if char.is_whitespace() {
                if char == '\n' {
                    line += 1;
                    column = 0;
                } else {
                    column += char.len_utf8();
                }
                offset += char.len_utf8();
                continue;
            }

            if char.is_ascii_digit() {
                let start = offset;
                offset += char.len_utf8();
                let mut after_column = column + char.len_utf8();

                while let Some(char) = chars.next() {
                    if !char.is_ascii_digit() {
                        break;
                    }

                    offset += char.len_utf8();
                    after_column += char.len_utf8();
                }

                let number = u32::from_str_radix(&self.source[start..offset], 10).unwrap();
                return (offset, Some(ExpressionToken::Atom(ExpressionAtom::NumericLiteral(number))), line, after_column);
            }

            if char.is_alphanumeric() {
                if let Some((length, symbol)) = self.lexer.symbols.starts_with(&self.source[offset..]) {
                    let mut after_line = line;
                    let mut after_column = column;
                    for char in self.source[offset..].chars() {
                        if char == '\n' {
                            after_line += 1;
                            after_column = 0;
                        } else {
                            after_column += char.len_utf8();
                        }
                    }

                    return (offset + length, Some(ExpressionToken::Atom(ExpressionAtom::Reference(symbol))), after_line, after_column);
                }

                let start = offset;
                offset += char.len_utf8();
                let mut after_column = column + char.len_utf8();

                while let Some(char) = chars.next() {
                    if !char.is_alphanumeric() {
                        break;
                    }

                    offset += char.len_utf8();
                    after_column += char.len_utf8();
                }

                let word = &self.source[start..offset].trim().to_lowercase();
                return (offset, match word.as_str() {
                    "no" | "false" | "never" => Some(ExpressionToken::Atom(ExpressionAtom::LogicalLiteral(false))),
                    "yes" | "true" | "always" => Some(ExpressionToken::Atom(ExpressionAtom::LogicalLiteral(false))),
                    "and" => Some(ExpressionToken::Operator(ExpressionOperator::And)),
                    "or" => Some(ExpressionToken::Operator(ExpressionOperator::Or)),
                    "not" => Some(ExpressionToken::Operator(ExpressionOperator::Not)),
                    "between" => Some(ExpressionToken::Operator(ExpressionOperator::Between)),
                    "maximum" => Some(ExpressionToken::Operator(ExpressionOperator::Maximum)),
                    "minimum" => Some(ExpressionToken::Operator(ExpressionOperator::Minimum)),
                    "any" | "either" | "one" | "among" => Some(ExpressionToken::Operator(ExpressionOperator::Either)),
                    "random" => Some(ExpressionToken::Operator(ExpressionOperator::Random)),
                    "is" => Some(ExpressionToken::Operator(ExpressionOperator::Is)),
                    "in" => Some(ExpressionToken::Operator(ExpressionOperator::In)),
                    "of" => Some(ExpressionToken::Operator(ExpressionOperator::Of)),
                    "then" => Some(ExpressionToken::Operator(ExpressionOperator::Then)),
                    "else" | "otherwise" => Some(ExpressionToken::Operator(ExpressionOperator::Else)),
                    "end" => Some(ExpressionToken::Operator(ExpressionOperator::End)),
                    "when" | "if" => Some(ExpressionToken::Operator(ExpressionOperator::When)),
                    "unless" => Some(ExpressionToken::Operator(ExpressionOperator::Unless)),
                    "max" | "greater" | "greatest" => Some(ExpressionToken::Operator(ExpressionOperator::Maximum)),
                    "min" | "lesser" | "least" => Some(ExpressionToken::Operator(ExpressionOperator::Minimum)),
                    _ => Some(ExpressionToken::UnrecognizedToken)
                }, line, after_column);
            }

            offset += char.len_utf8();
            let after_column = column + char.len_utf8();
            let token = match char {
                '(' => Some(ExpressionToken::Operator(ExpressionOperator::OpenParen)),
                ')' => Some(ExpressionToken::Operator(ExpressionOperator::CloseParen)),
                ',' => Some(ExpressionToken::Operator(ExpressionOperator::Comma)),
                '+' => Some(ExpressionToken::Operator(ExpressionOperator::Plus)),
                '-' => Some(ExpressionToken::Operator(ExpressionOperator::Minus)),
                '*' => Some(ExpressionToken::Operator(ExpressionOperator::Multiply)),
                '/' => Some(ExpressionToken::Operator(ExpressionOperator::Divide)),
                '!' => {
                    if let Some(char) = chars.next() {
                        if char == '=' {
                            offset += char.len_utf8();

                            while let Some(char) = chars.next() {
                                if char != '=' {
                                    break;
                                }

                                offset += char.len_utf8();
                            }

                            Some(ExpressionToken::Operator(ExpressionOperator::NotEqual))
                        } else {
                            Some(ExpressionToken::Operator(ExpressionOperator::Not))
                        }
                    } else {
                        Some(ExpressionToken::Operator(ExpressionOperator::Not))
                    }
                },
                '>' => {
                    if let Some(char) = chars.next() {
                        if char == '=' {
                            offset += char.len_utf8();

                            while let Some(char) = chars.next() {
                                if char != '=' {
                                    break;
                                }

                                offset += char.len_utf8();
                            }

                            Some(ExpressionToken::Operator(ExpressionOperator::GreaterThanOrEqual))
                        } else {
                            Some(ExpressionToken::Operator(ExpressionOperator::GreaterThan))
                        }
                    } else {
                        Some(ExpressionToken::Operator(ExpressionOperator::GreaterThan))
                    }
                },
                '<' => {
                    if let Some(char) = chars.next() {
                        if char == '=' {
                            offset += char.len_utf8();

                            while let Some(char) = chars.next() {
                                if char != '=' {
                                    break;
                                }

                                offset += char.len_utf8();
                            }

                            Some(ExpressionToken::Operator(ExpressionOperator::LessThanOrEqual))
                        } else {
                            Some(ExpressionToken::Operator(ExpressionOperator::LessThan))
                        }
                    } else {
                        Some(ExpressionToken::Operator(ExpressionOperator::LessThan))
                    }
                },
                '=' => {
                    while let Some(char) = chars.next() {
                        if char != '=' {
                            break;
                        }

                        offset += char.len_utf8();
                    }

                    Some(ExpressionToken::Operator(ExpressionOperator::Equal))
                },
                _ => Some(ExpressionToken::UnrecognizedToken)
            };

            return (offset, token, line, after_column)
        }

        (offset, None, line, column)
    }
}
