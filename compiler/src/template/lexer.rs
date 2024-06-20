use std::fmt::{Debug, Formatter};
use crate::{Attribution, Mark};
use crate::expression::{ExpressionLex, ExpressionLexer};
use crate::symbol::{SymbolList};

pub enum TemplateToken<'a> {
    Text(String),
    Delimiter(Delimiter),
    ParagraphBreak,
    Tag(ExpressionLex<'a>),
    AnchorBegin,
    AnchorEnd(String),
}

impl<'a> Debug for TemplateToken<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => f.write_str(&format!("Text({:?})", s)),
            Self::Delimiter(d) => f.write_str(&format!("Delimiter({:?})", d)),
            Self::ParagraphBreak => f.write_str("ParagraphBreak"),
            Self::Tag(_) => f.write_str("Tag"),
            Self::AnchorBegin => f.write_str("AnchorBegin"),
            Self::AnchorEnd(s) => f.write_str(&format!("AnchorEnd({:?})", s)),
        }
    }
}

impl<'a> PartialEq for TemplateToken<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Text(s) => if let Self::Text(s2) = other {
                s == s2
            } else {
                false
            },
            Self::Delimiter(d) => if let Self::Delimiter(d2) = other {
                d == d2
            } else {
                false
            },
            Self::ParagraphBreak => if let Self::ParagraphBreak = other { true } else { false },
            Self::Tag(_) => if let Self::Tag(_) = other { true } else { false },
            Self::AnchorBegin => if let Self::AnchorBegin = other { true } else { false },
            Self::AnchorEnd(s) => if let Self::AnchorEnd(s2) = other {
                s == s2
            } else {
                false
            },
        }
    }
}

impl<'a> Eq for TemplateToken<'a> {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Delimiter {
    pub character: char,
    pub length: usize,
    pub opener: bool,
    pub closer: bool,
}

pub struct TemplateLexer<'a> {
    expression_lexer: ExpressionLexer<'a>,
}

pub struct TemplateLex<'a> {
    lexer: &'a TemplateLexer<'a>,
    source: &'a str,
    #[allow(dead_code)]
    attribution: &'a Attribution,
    line: usize,
    column: usize,
    state: TemplateLexerState,
    string: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TemplateLexerState {
    Initial,
    Whitespace,
    Newline,
    DoubleNewline,
    Punctuation,
    Text,
}

impl<'a> TemplateLexer<'a> {
    pub fn new(symbols: &'a SymbolList) -> Self {
        TemplateLexer {
            expression_lexer: ExpressionLexer::new(symbols),
        }
    }

    pub fn lex(&'a self, source: &'a str, attribution: &'a Attribution) -> TemplateLex<'a> {
        TemplateLex {
            lexer: self,
            source,
            attribution,
            line: 0,
            column: 0,
            state: TemplateLexerState::Initial,
            string: String::new(),
        }
    }
}

impl<'a> TemplateLex<'a> {
    pub fn mark(&self) -> Mark {
        Mark {
            line: self.line as u64,
            column: self.column as u64,
        }
    }

    pub fn next(&mut self) -> Option<TemplateToken<'a>> {
        let mut offset = 0usize;

        let mut chars = self.source.chars();

        while let Some(char) = chars.next() {
            if char.is_whitespace() {
                offset += char.len_utf8();
                if char == '\n' {
                    self.line += 1;
                    self.column = 0;
                } else {
                    self.column += char.len_utf8();
                }
                match self.state {
                    TemplateLexerState::Initial | TemplateLexerState::DoubleNewline => {},
                    TemplateLexerState::Whitespace => {
                        if char == '\n' {
                            self.state = TemplateLexerState::Newline;
                        }
                    },
                    TemplateLexerState::Newline => {
                        if char == '\n' {
                            self.state = TemplateLexerState::DoubleNewline;
                            if !self.string.is_empty() {
                                let string = std::mem::replace(&mut self.string, String::new());
                                self.source = &self.source[offset..];
                                return Some(TemplateToken::Text(string));
                            }
                        }
                    },
                    _ => {
                        if char == '\n' {
                            self.state = TemplateLexerState::Newline;
                        } else {
                            self.state = TemplateLexerState::Whitespace;
                        }
                    },
                }
            } else {
                match self.state {
                    TemplateLexerState::Whitespace | TemplateLexerState::Newline => {
                        self.string.push(' ');
                        self.state = TemplateLexerState::Initial;
                    },
                    TemplateLexerState::DoubleNewline => {
                        self.source = &self.source[offset..];
                        self.state = TemplateLexerState::Initial;
                        return Some(TemplateToken::ParagraphBreak);
                    },
                    _ => {},
                }

                match char {
                    '*' | '_' | '{' | '[' | ']' => {
                        if !self.string.is_empty() {
                            let string = std::mem::replace(&mut self.string, String::new());
                            self.source = &self.source[offset..];
                            return Some(TemplateToken::Text(string));
                        }
                    },
                    _ => {
                        offset += char.len_utf8();
                        self.column += char.len_utf8();
                        self.string.push(char);
                        if char.is_ascii_punctuation() {
                            self.state = TemplateLexerState::Punctuation;
                        } else {
                            self.state = TemplateLexerState::Text;
                        }
                        continue;
                    },
                }

                match char {
                    '[' => {
                        offset += char.len_utf8();
                        self.source = &self.source[offset..];
                        return Some(TemplateToken::AnchorBegin)
                    },
                    ']' => {
                        offset += char.len_utf8();
                        self.column += char.len_utf8();
                        if let Some('(') = chars.next() {
                            offset += '('.len_utf8();
                            self.column += char.len_utf8();
                            let start = offset;
                            let mut escape = false;
                            while let Some(char) = chars.next() {
                                if char == '\\' {
                                    offset += char.len_utf8();
                                    self.column += char.len_utf8();
                                    escape = true;
                                } else if escape {
                                    offset += char.len_utf8();
                                    if char == '\n' {
                                        self.line += 1;
                                        self.column = 0;
                                    } else {
                                        self.column += char.len_utf8();
                                    }
                                } else if char == '\n' {
                                    offset += char.len_utf8();
                                    self.line += 1;
                                    self.column = 0;
                                    self.state = TemplateLexerState::Newline;
                                    break;
                                } else if char == ')' {
                                    let href = &self.source[start..offset];
                                    offset += char.len_utf8();
                                    self.column += char.len_utf8();
                                    self.source = &self.source[offset..];
                                    return Some(TemplateToken::AnchorEnd(String::from(href.trim())));
                                } else {
                                    offset += char.len_utf8();
                                    self.column += char.len_utf8();
                                }
                            }
                        }
                    },
                    '*' => {
                        offset += char.len_utf8();
                        self.column += char.len_utf8();
                        let mut delimiter_length = 1;
                        let whitespace_precedes = match self.state {
                            TemplateLexerState::Initial | TemplateLexerState::Newline | TemplateLexerState::DoubleNewline | TemplateLexerState::Whitespace => true,
                            _ => false,
                        };
                        let punctuation_precedes = if let TemplateLexerState::Punctuation = self.state { true } else { false };
                        let mut whitespace_follows = true;
                        let mut punctuation_follows = false;
                        while let Some(char) = chars.next() {
                            if char != '*' {
                                whitespace_follows = char.is_whitespace();
                                punctuation_follows = char.is_ascii_punctuation();
                                break;
                            }

                            offset += char.len_utf8();
                            self.column += char.len_utf8();
                            delimiter_length += 1;
                        }

                        let left_flanking = !whitespace_follows && (
                            !punctuation_follows || whitespace_precedes || punctuation_precedes);
                        let right_flanking = !whitespace_precedes && (
                            !punctuation_precedes || whitespace_follows || punctuation_follows);
                        self.source = &self.source[offset..];
                        return Some(TemplateToken::Delimiter(Delimiter {
                            character: '*',
                            length: delimiter_length,
                            opener: left_flanking,
                            closer: right_flanking,
                        }));
                    },
                    '_' => {
                        offset += char.len_utf8();
                        self.column += char.len_utf8();
                        let mut delimiter_length = 1;
                        let whitespace_precedes = match self.state {
                            TemplateLexerState::Initial | TemplateLexerState::Newline | TemplateLexerState::DoubleNewline | TemplateLexerState::Whitespace => true,
                            _ => false,
                        };
                        let punctuation_precedes = if let TemplateLexerState::Punctuation = self.state { true } else { false };
                        let mut whitespace_follows = true;
                        let mut punctuation_follows = false;
                        while let Some(char) = chars.next() {
                            if char != '_' {
                                whitespace_follows = char.is_whitespace();
                                punctuation_follows = char.is_ascii_punctuation();
                                break;
                            }

                            offset += char.len_utf8();
                            self.column += char.len_utf8();
                            delimiter_length += 1;
                        }

                        let left_flanking = !whitespace_follows && (
                            !punctuation_follows || whitespace_precedes || punctuation_precedes);
                        let right_flanking = !whitespace_precedes && (
                            !punctuation_precedes || whitespace_follows || punctuation_follows);
                        self.source = &self.source[offset..];
                        return Some(TemplateToken::Delimiter(Delimiter {
                            character: '_',
                            length: delimiter_length,
                            opener: left_flanking && (!right_flanking || punctuation_precedes),
                            closer: right_flanking && (!left_flanking || punctuation_follows),
                        }));
                    },
                    '{' => {
                        offset += char.len_utf8();
                        self.column += char.len_utf8();
                        let start_mark = Mark {
                            line: self.line as u64,
                            column: self.column as u64,
                        };
                        let start = offset;
                        while let Some(char) = chars.next() {
                            if char == '}' {
                                break;
                            }

                            offset += char.len_utf8();
                            if char == '\n' {
                                self.line += 1;
                                self.column = 0;
                            } else {
                                self.column += char.len_utf8();
                            }
                        }
                        let tag_source = &self.source[start..offset];
                        offset += '}'.len_utf8();
                        self.column += '}'.len_utf8();
                        self.source = &self.source[offset.min(self.source.len())..];
                        return Some(TemplateToken::Tag(self.lexer.expression_lexer.lex_at_mark(tag_source, start_mark)));
                    },
                    _ => {},
                }
            }
        }

        if !self.string.is_empty() {
            let string = std::mem::replace(&mut self.string, String::new());
            self.source = &self.source[offset..];
            return Some(TemplateToken::Text(string));
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::expression::{ExpressionAtom, ExpressionOperator, ExpressionToken, MarkedToken};
    use super::*;

    #[test]
    pub fn test_template_lexer() {
        let symbols = SymbolList::builder().push("a river in space").push("a river in time").build();
        let lexer = TemplateLexer::new(&symbols);
        let attribution = Attribution::new("test", Mark { line: 0, column: 0 }, Mark { line: 0, column: 0 });

        let mut lex = lexer.lex("I'm *serious*\n\n{if a river in time}Okay?{else} Never\nmind{end}  Bye!", &attribution);
        assert_eq!(lex.next(), Some(TemplateToken::Text(String::from("I'm "))));
        assert_eq!(lex.next(), Some(TemplateToken::Delimiter(Delimiter {
            character: '*',
            length: 1,
            opener: true,
            closer: false,
        })));
        assert_eq!(lex.next(), Some(TemplateToken::Text(String::from("serious"))));
        assert_eq!(lex.next(), Some(TemplateToken::Delimiter(Delimiter {
            character: '*',
            length: 1,
            opener: false,
            closer: true,
        })));
        assert_eq!(lex.next(), Some(TemplateToken::ParagraphBreak));
        if let Some(TemplateToken::Tag(mut tag_lex)) = lex.next() {
            assert_eq!(tag_lex.next(), Some(MarkedToken {
                token: ExpressionToken::Operator(ExpressionOperator::When),
                start_mark: Mark { line: 2, column: 1 },
                end_mark: Mark { line: 2, column: 3 },
            }));
            assert_eq!(tag_lex.next(), Some(MarkedToken {
                token: ExpressionToken::Atom(ExpressionAtom::Reference(String::from("a river in time"))),
                start_mark: Mark { line: 2, column: 3},
                end_mark: Mark { line: 2, column: 19},
            }));
            assert_eq!(tag_lex.next(), None);
        } else {
            assert!(false);
        }
        assert_eq!(lex.next(), Some(TemplateToken::Text(String::from("Okay?"))));
        if let Some(TemplateToken::Tag(mut tag_lex)) = lex.next() {
            assert_eq!(tag_lex.next(), Some(MarkedToken {
                token: ExpressionToken::Operator(ExpressionOperator::Else),
                start_mark: Mark { line: 2, column: 26 },
                end_mark: Mark { line: 2, column: 30 },
            }));
            assert_eq!(tag_lex.next(), None);
        }
        assert_eq!(lex.next(), Some(TemplateToken::Text(String::from(" Never mind"))));
        if let Some(TemplateToken::Tag(mut tag_lex)) = lex.next() {
            assert_eq!(tag_lex.next(), Some(MarkedToken {
                token: ExpressionToken::Operator(ExpressionOperator::End),
                start_mark: Mark { line: 3, column: 5 },
                end_mark: Mark { line: 3, column: 8 },
            }));
            assert_eq!(tag_lex.next(), None);
        }
        assert_eq!(lex.next(), Some(TemplateToken::Text(String::from(" Bye!"))));
    }

    #[test]
    pub fn it_lexes_anchors() {
        let symbols = SymbolList::new();
        let lexer = TemplateLexer::new(&symbols);
        let attribution = Attribution::new("test", Mark { line: 0, column: 0 }, Mark { line: 0, column: 0 });

        let mut lex = lexer.lex("Sometimes the answer lies [elsewhere](https://example.com).", &attribution);
        assert_eq!(lex.next(), Some(TemplateToken::Text(String::from("Sometimes the answer lies "))));
        assert_eq!(lex.next(), Some(TemplateToken::AnchorBegin));
        assert_eq!(lex.next(), Some(TemplateToken::Text(String::from("elsewhere"))));
        assert_eq!(lex.next(), Some(TemplateToken::AnchorEnd(String::from("https://example.com"))));
        assert_eq!(lex.next(), Some(TemplateToken::Text(String::from("."))));
    }
}
