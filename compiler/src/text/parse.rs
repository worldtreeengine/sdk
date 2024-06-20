use crate::{Problem};
use crate::text::{Text, TextNode};
use crate::text::lexer::{Delimiter, TextLex, TextLexer, TextToken};

pub struct TextParser {
    lexer: TextLexer,
}

impl TextParser {
    pub fn new() -> Self {
        TextParser {
            lexer: TextLexer::new(),
        }
    }

    pub fn parse(&self, source: &str) -> TextParsingResult {
        Parse::new(self, source).parse()
    }
}

struct Parse<'a> {
    #[allow(dead_code)]
    parser: &'a TextParser,
    lex: TextLex<'a>,
    problems: Vec<Problem>
}

pub struct TextParsingResult {
    pub text: Text,
    pub problems: Vec<Problem>,
}

#[derive(Clone, Copy)]
struct DelimiterPointer {
    delimiter: Delimiter,
    index: usize,
}

impl<'a> Parse<'a> {
    fn new(parser: &'a TextParser, source: &'a str) -> Self {
        Self {
            parser,
            lex: parser.lexer.lex(source),
            problems: Vec::new(),
        }
    }

    fn parse(mut self) -> TextParsingResult {
        let text = self.parse_inner();

        TextParsingResult {
            text,
            problems: self.problems,
        }
    }

    fn process_styles(&mut self, parse: &mut Text, mut delimiter_stack: Vec<DelimiterPointer>) {
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
                            parse.insert(earlier_pointer.index, TextNode::Bold(inner));

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
                            parse.insert(earlier_pointer.index, TextNode::Italic(inner));

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

    fn parse_inner(&mut self) -> Text {
        let mut paragraphs = Vec::new();
        let mut parse = Vec::new();

        let mut delimiter_stack = Vec::new();

        while let Some(token) = self.lex.next() {
            match token {
                TextToken::Text(t) => {
                    parse.push(TextNode::Plain(t));
                },
                TextToken::ParagraphBreak => {
                    self.process_styles(&mut parse, std::mem::replace(&mut delimiter_stack, Vec::new()));
                    paragraphs.push(TextNode::Paragraph(std::mem::replace(&mut parse, Vec::new())));
                },
                TextToken::Delimiter(delimiter) => {
                    delimiter_stack.push(DelimiterPointer {
                        delimiter,
                        index: parse.len(),
                    });
                },
                TextToken::AnchorBegin => {
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
                TextToken::AnchorEnd(href) => {
                    for i in (0..delimiter_stack.len()).rev() {
                        let delimiter = delimiter_stack[i].clone();
                        if delimiter.delimiter.character == '[' {
                            self.process_styles(&mut parse, delimiter_stack.splice(i.., None).collect());
                            let content = parse.splice(delimiter.index.., None).collect();
                            parse.push(TextNode::Anchor(href, content));
                            break;
                        }
                    }
                },
            }
        }

        self.process_styles(&mut parse, delimiter_stack);
        paragraphs.push(TextNode::Paragraph(parse));

        paragraphs
    }
}
