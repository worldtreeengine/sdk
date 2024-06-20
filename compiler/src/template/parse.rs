use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use crate::{Attribution, Problem};
use crate::expression::{ExpressionParse, ExpressionParser, TagParse};
use crate::symbol::{SymbolList};
use crate::template::lexer::{Delimiter, TemplateLex, TemplateLexer, TemplateToken};

pub type TemplateParse = Vec<TemplateParseNode>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TemplateParseNode {
    Text(String),
    Paragraph,
    Italic(Vec<TemplateParseNode>),
    Bold(Vec<TemplateParseNode>),
    Anchor(String, Vec<TemplateParseNode>),
    Branch(ExpressionParse, Vec<TemplateParseNode>, Option<Vec<TemplateParseNode>>),
}

impl Serialize for TemplateParseNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Self::Text(s) => serializer.serialize_str(s),
            Self::Paragraph => serializer.serialize_str("\n"),
            Self::Italic(t) => {
                let mut mapping = serializer.serialize_map(Some(1))?;
                mapping.serialize_entry("i", t)?;
                mapping.end()
            },
            Self::Bold(t) => {
                let mut mapping = serializer.serialize_map(Some(1))?;
                mapping.serialize_entry("b", t)?;
                mapping.end()
            },
            Self::Anchor(href, a) => {
                let mut mapping = serializer.serialize_map(Some(2))?;
                mapping.serialize_entry("a", a)?;
                mapping.serialize_entry("href", href)?;
                mapping.end()
            },
            Self::Branch(condition, then, next) => {
                if let Some(next) = next {
                    let mut mapping = serializer.serialize_map(Some(3))?;
                    mapping.serialize_entry("condition", condition)?;
                    mapping.serialize_entry("value", then)?;
                    mapping.serialize_entry("next", next)?;
                    mapping.end()
                } else {
                    let mut mapping = serializer.serialize_map(Some(2))?;
                    mapping.serialize_entry("condition", condition)?;
                    mapping.serialize_entry("value", then)?;
                    mapping.end()
                }
            },
        }
    }
}

pub struct TemplateParser<'a> {
    lexer: TemplateLexer<'a>,
    expression_parser: ExpressionParser<'a>,
}

struct Parse<'a> {
    parser: &'a TemplateParser<'a>,
    lex: TemplateLex<'a>,
    attribution: &'a Attribution,
    problems: Vec<Problem>,
}

#[derive(Clone, Copy)]
struct DelimiterPointer {
    delimiter: Delimiter,
    index: usize,
}

pub struct TemplateParsingResult {
    pub parse: Vec<TemplateParseNode>,
    pub problems: Vec<Problem>,
}

impl<'a> TemplateParser<'a> {
    pub fn new(symbols: &'a SymbolList) -> Self {
        Self {
            lexer: TemplateLexer::new(symbols),
            expression_parser: ExpressionParser::new(symbols),
        }
    }

    pub fn parse(&self, source: &str, attribution: &Attribution) -> TemplateParsingResult {
        Parse::new(self, source, attribution).parse()
    }
}

impl<'a> Parse<'a> {
    fn new(parser: &'a TemplateParser, source: &'a str, attribution: &'a Attribution) -> Self {
        Self {
            parser,
            lex: parser.lexer.lex(source, attribution),
            attribution,
            problems: Vec::new(),
        }
    }

    fn parse(mut self) -> TemplateParsingResult {
        let (parse, _) = self.parse_inner(true, false);

        TemplateParsingResult {
            parse,
            problems: self.problems,
        }
    }

    fn process_styles(&mut self, parse: &mut TemplateParse, mut delimiter_stack: Vec<DelimiterPointer>) {
        /*
           Adapted from the algorithm described in the CommonMark spec https://spec.commonmark.org/0.31.2/#phase-2-inline-structure
        */
        let mut i = 0;
        while i < delimiter_stack.len() {
            let mut pointer = delimiter_stack[i];
            if pointer.delimiter.closer && pointer.delimiter.length > 0 {
                let mut j = i - 1;
                loop {
                    let mut earlier_pointer = delimiter_stack[j];
                    if earlier_pointer.delimiter.opener && earlier_pointer.delimiter.character == pointer.delimiter.character && earlier_pointer.delimiter.length > 0 {
                        delimiter_stack.splice(j + 1..i, None);
                        j = i - 1;

                        if pointer.delimiter.length >= 2 && earlier_pointer.delimiter.length >= 2 {
                            let inner = parse.splice(earlier_pointer.index..pointer.index, None).collect();
                            parse.insert(earlier_pointer.index, TemplateParseNode::Bold(inner));

                            pointer.index -= pointer.index - earlier_pointer.index - 1;

                            earlier_pointer.delimiter.length = 1.min(earlier_pointer.delimiter.length - 2);
                            pointer.delimiter.length = 1.min(pointer.delimiter.length - 2);
                            delimiter_stack[j] = earlier_pointer;
                            delimiter_stack[i] = pointer;

                            for k in i + 1..delimiter_stack.len() {
                                let mut future_pointer = delimiter_stack[k];
                                future_pointer.index -= pointer.index - earlier_pointer.index - 1;
                                delimiter_stack[k] = future_pointer;
                            }
                        }

                        if pointer.delimiter.length > 0 && earlier_pointer.delimiter.length > 0 {
                            let inner = parse.splice(earlier_pointer.index..pointer.index, None).collect();
                            parse.insert(earlier_pointer.index, TemplateParseNode::Italic(inner));

                            pointer.index -= pointer.index - earlier_pointer.index - 1;

                            earlier_pointer.delimiter.length = 0;
                            pointer.delimiter.length = 0;
                            delimiter_stack[j] = earlier_pointer;
                            delimiter_stack[i] = pointer;

                            for k in i + 1..delimiter_stack.len() {
                                let mut future_pointer = delimiter_stack[k];
                                future_pointer.index -= pointer.index - earlier_pointer.index - 1;
                                delimiter_stack[k] = future_pointer;
                            }
                        }

                        if pointer.delimiter.length < 1 {
                            break;
                        }
                    }
                    if j == 0 {
                        break;
                    } else {
                        j -= 1;
                    }
                }
            }
            i += 1;
        }
    }

    fn parse_inner(&mut self, paragraph_before: bool, in_branch: bool) -> (TemplateParse, Option<TagParse>) {
        let mut parse = Vec::new();

        let mut delimiter_stack = Vec::new();
        let mut paragraph_precedes = paragraph_before;

        while let Some(token) = self.lex.next() {
            match token {
                TemplateToken::Text(t) => {
                    paragraph_precedes = false;
                    parse.push(TemplateParseNode::Text(t));
                },
                TemplateToken::ParagraphBreak => {
                    paragraph_precedes = true;
                    parse.push(TemplateParseNode::Paragraph);
                    self.process_styles(&mut parse, std::mem::replace(&mut delimiter_stack, Vec::new()));
                },
                TemplateToken::Delimiter(delimiter) => {
                    paragraph_precedes = false;
                    delimiter_stack.push(DelimiterPointer {
                        delimiter,
                        index: parse.len(),
                    });
                },
                TemplateToken::Tag(mut expression_lex) => {
                    let tag_parsing_result = self.parser.expression_parser.parse_tag(&mut expression_lex, &self.attribution);
                    if let Some(tag_parse) = tag_parsing_result.parse {
                        self.problems.extend(tag_parsing_result.problems);
                        match tag_parse {
                            TagParse::When(condition) => {
                                parse.push(self.parse_branch(paragraph_precedes, condition));
                            },
                            _ => {
                                if !in_branch {
                                    self.problems.push(Problem::fatal("Invalid tag", &self.attribution.at_mark(self.lex.mark())));
                                } else {
                                    return (parse, Some(tag_parse));
                                }
                            },
                        }
                    } else {
                        if tag_parsing_result.problems.is_empty() {
                            self.problems.push(Problem::fatal("Empty tag", &self.attribution.at_marks(expression_lex.mark(), expression_lex.mark())));
                        } else {
                            self.problems.extend(tag_parsing_result.problems);
                        }
                    }
                },
                TemplateToken::AnchorBegin => {
                    delimiter_stack.push(DelimiterPointer {
                        index: parse.len(),
                        delimiter: Delimiter {
                            character: '[',
                            length: 1,
                            opener: false,
                            closer: false,
                        },
                    });
                },
                TemplateToken::AnchorEnd(href) => {
                    for i in (0..delimiter_stack.len()).rev() {
                        let delimiter = delimiter_stack[i].clone();
                        if delimiter.delimiter.character == '[' {
                            self.process_styles(&mut parse, delimiter_stack.splice(i.., None).collect());
                            let content = parse.splice(delimiter.index.., None).collect();
                            parse.push(TemplateParseNode::Anchor(href, content));
                            break;
                        }
                    }
                },
            }
        }

        self.process_styles(&mut parse, delimiter_stack);

        (parse, None)
    }

    fn parse_branch(&mut self, paragraph_before: bool, condition: ExpressionParse) -> TemplateParseNode {
        let (if_branch, tag_parse) = self.parse_inner(paragraph_before, true);
        if let Some(tag_parse) = tag_parse {
            match tag_parse {
                TagParse::When(else_condition) | TagParse::Else(Some(else_condition)) => {
                    let else_branch = self.parse_branch(paragraph_before, else_condition);
                    TemplateParseNode::Branch(condition, if_branch, Some(vec!(else_branch)))
                },
                TagParse::Else(None) => {
                    let (else_branch, end_tag_parse) = self.parse_inner(paragraph_before, true);
                    match end_tag_parse {
                        Some(TagParse::End) => {}
                        _ => {
                            self.problems.push(Problem::fatal("End tag expected", &self.attribution.at_mark(self.lex.mark())));
                        }
                    }
                    TemplateParseNode::Branch(condition, if_branch, Some(else_branch))
                },
                TagParse::End => {
                    TemplateParseNode::Branch(condition, if_branch, None)
                },
            }
        } else {
            self.problems.push(Problem::fatal("End tag expected", &self.attribution.at_mark(self.lex.mark())));
            TemplateParseNode::Branch(condition, if_branch, None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::expression::ExpressionAtom;
    use crate::Mark;
    use super::*;

    #[test]
    pub fn test_template_parser() {
        let mut symbols = SymbolList::new();
        symbols.push("a river in time");
        let parser = TemplateParser::new(&symbols);
        let attribution = Attribution::new("test", Mark { line: 0, column: 0 }, Mark { line: 0, column: 0 });
        let result = parser.parse("I'm *serious*\n\n{if a river in time}Okay?{else} Never\nmind{end}  Bye!", &attribution);
        assert_eq!(result.parse, vec!(
            TemplateParseNode::Text(String::from("I'm ")),
            TemplateParseNode::Italic(vec!(
                TemplateParseNode::Text(String::from("serious")),
            )),
            TemplateParseNode::Paragraph,
            TemplateParseNode::Branch(
                ExpressionParse::Atom(
                    ExpressionAtom::Reference(String::from("a river in time")),
                ),
                vec!(
                    TemplateParseNode::Text(String::from("Okay?")),
                ),
                Some(vec!(
                    TemplateParseNode::Text(String::from(" Never mind")),
                )),
            ),
            TemplateParseNode::Text(String::from(" Bye!")),
        ));
    }
}

