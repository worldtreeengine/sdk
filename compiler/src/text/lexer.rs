use std::fmt::{Debug};
use crate::{Mark};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextToken {
    Text(String),
    Delimiter(Delimiter),
    ParagraphBreak,
    AnchorBegin,
    AnchorEnd(String),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Delimiter {
    pub character: char,
    pub length: usize,
    pub opener: bool,
    pub closer: bool,
}

pub struct TextLexer {}

pub struct TextLex<'a> {
    #[allow(dead_code)]
    lexer: &'a TextLexer,
    source: &'a str,
    line: usize,
    column: usize,
    state: TextLexerState,
    string: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TextLexerState {
    Initial,
    Whitespace,
    Newline,
    DoubleNewline,
    Punctuation,
    Text,
}

impl TextLexer {
    pub fn new() -> Self {
        TextLexer {}
    }

    pub fn lex<'a>(&'a self, source: &'a str) -> TextLex<'a> {
        TextLex {
            lexer: self,
            source,
            line: 0,
            column: 0,
            state: TextLexerState::Initial,
            string: String::new(),
        }
    }
}

impl<'a> TextLex<'a> {
    #[allow(dead_code)]
    pub fn mark(&self) -> Mark {
        Mark {
            line: self.line as u64,
            column: self.column as u64,
        }
    }

    pub fn next(&mut self) -> Option<TextToken> {
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
                    TextLexerState::Initial | TextLexerState::DoubleNewline => {},
                    TextLexerState::Whitespace => {
                        if char == '\n' {
                            self.state = TextLexerState::Newline;
                        }
                    },
                    TextLexerState::Newline => {
                        if char == '\n' {
                            self.state = TextLexerState::DoubleNewline;
                            if !self.string.is_empty() {
                                let string = std::mem::replace(&mut self.string, String::new());
                                self.source = &self.source[offset..];
                                return Some(TextToken::Text(string));
                            }
                        }
                    },
                    _ => {
                        if char == '\n' {
                            self.state = TextLexerState::Newline;
                        } else {
                            self.state = TextLexerState::Whitespace;
                        }
                    },
                }
            } else {
                match self.state {
                    TextLexerState::Whitespace | TextLexerState::Newline => {
                        self.string.push(' ');
                        self.state = TextLexerState::Initial;
                    },
                    TextLexerState::DoubleNewline => {
                        self.source = &self.source[offset..];
                        self.state = TextLexerState::Initial;
                        return Some(TextToken::ParagraphBreak);
                    },
                    _ => {},
                }

                match char {
                    '*' | '_' | '[' | ']' => {
                        if !self.string.is_empty() {
                            let string = std::mem::replace(&mut self.string, String::new());
                            self.source = &self.source[offset..];
                            return Some(TextToken::Text(string));
                        }
                    },
                    _ => {
                        offset += char.len_utf8();
                        self.column += char.len_utf8();
                        self.string.push(char);
                        if char.is_ascii_punctuation() {
                            self.state = TextLexerState::Punctuation;
                        } else {
                            self.state = TextLexerState::Text;
                        }
                        continue;
                    },
                }

                match char {
                    '[' => {
                        offset += char.len_utf8();
                        self.source = &self.source[offset..];
                        return Some(TextToken::AnchorBegin)
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
                                    self.state = TextLexerState::Newline;
                                    break;
                                } else if char == ')' {
                                    let href = &self.source[start..offset];
                                    offset += char.len_utf8();
                                    self.column += char.len_utf8();
                                    self.source = &self.source[offset..];
                                    return Some(TextToken::AnchorEnd(String::from(href.trim())));
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
                            TextLexerState::Initial | TextLexerState::Newline | TextLexerState::DoubleNewline | TextLexerState::Whitespace => true,
                            _ => false,
                        };
                        let punctuation_precedes = if let TextLexerState::Punctuation = self.state { true } else { false };
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
                        return Some(TextToken::Delimiter(Delimiter {
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
                            TextLexerState::Initial | TextLexerState::Newline | TextLexerState::DoubleNewline | TextLexerState::Whitespace => true,
                            _ => false,
                        };
                        let punctuation_precedes = if let TextLexerState::Punctuation = self.state { true } else { false };
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
                        return Some(TextToken::Delimiter(Delimiter {
                            character: '_',
                            length: delimiter_length,
                            opener: left_flanking && (!right_flanking || punctuation_precedes),
                            closer: right_flanking && (!left_flanking || punctuation_follows),
                        }));
                    },
                    _ => {},
                }
            }
        }

        if !self.string.is_empty() {
            let string = std::mem::replace(&mut self.string, String::new());
            self.source = &self.source[offset..];
            return Some(TextToken::Text(string));
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_text_lexer() {
        let lexer = TextLexer::new();
        let mut lex = lexer.lex("I'm *serious*\n\n{if a river in time}Okay?{else} Never\nmind{end}  Bye!");
        assert_eq!(lex.next(), Some(TextToken::Text(String::from("I'm "))));
        assert_eq!(lex.next(), Some(TextToken::Delimiter(Delimiter {
            character: '*',
            length: 1,
            opener: true,
            closer: false,
        })));
        assert_eq!(lex.next(), Some(TextToken::Text(String::from("serious"))));
        assert_eq!(lex.next(), Some(TextToken::Delimiter(Delimiter {
            character: '*',
            length: 1,
            opener: false,
            closer: true,
        })));
        assert_eq!(lex.next(), Some(TextToken::ParagraphBreak));
        assert_eq!(lex.next(), Some(TextToken::Text(String::from("{if a river in time}Okay?{else} Never mind{end} Bye!"))));
    }

    #[test]
    pub fn it_lexes_anchors() {
        let lexer = TextLexer::new();
        let mut lex = lexer.lex("Sometimes the answer lies [elsewhere](https://example.com).");
        assert_eq!(lex.next(), Some(TextToken::Text(String::from("Sometimes the answer lies "))));
        assert_eq!(lex.next(), Some(TextToken::AnchorBegin));
        assert_eq!(lex.next(), Some(TextToken::Text(String::from("elsewhere"))));
        assert_eq!(lex.next(), Some(TextToken::AnchorEnd(String::from("https://example.com"))));
        assert_eq!(lex.next(), Some(TextToken::Text(String::from("."))));
    }
}
