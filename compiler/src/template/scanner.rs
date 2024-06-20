// use lazy_static::lazy_static;
// use regex::Regex;
// use crate::expression::{ExpressionLexer, ExpressionParser, TagParse};
// use crate::{Attribution, Problem};
// use crate::symbol::SymbolTable;
//
// lazy_static! {
//     static ref WHITESPACE_REGEX: Regex = Regex::new("\\s+").unwrap();
// }
//
// fn scan_template(source: &str, symbols: &mut SymbolTable, problems: &mut Vec<Problem>, should_obfuscate: bool, attribution: &Attribution) -> Vec<TemplateToken> {
//     let mut tokens = Vec::new();
//     let mut expression_lexer = ExpressionLexer::new(symbols);
//     let mut expression_parser = ExpressionParser::new();
//
//     let chars = source.chars();
//     let mut start = 0;
//     let mut offset = 0;
//
//     let mut in_tag = false;
//
//     let mut delimiter_type = None;
//     let mut delimiter_length = 0;
//     let mut whitespace_precedes = true;
//     let mut punctuation_precedes = false;
//
//     let mut newline_count = 0;
//
//     for char in chars {
//         offset += char.len_utf8();
//
//         if in_tag {
//             if char == '}' {
//                 let inner_source = &source[start..offset - char.len_utf8()];
//                 if let Some(tag) = expression_parser.parse_tag(&mut expression_lexer.lex(inner_source), attribution).parse {
//                     tokens.push(TemplateToken::Tag(tag));
//                 } else {
//                     problems.push(Problem::fatal("Unrecognized tag", attribution));
//                 }
//                 in_tag = false;
//                 whitespace_precedes = true;
//                 newline_count = 0;
//                 start = offset;
//             }
//             continue;
//         }
//
//         if let Some(d) = delimiter_type {
//             match d {
//                 /*
//                     Adapted from the algorithm described in the CommonMark spec https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis
//                  */
//                 '*' => {
//                     if char == '*' {
//                         delimiter_length += 1;
//                         continue;
//                     }
//
//                     let whitespace_follows = char.is_whitespace();
//                     let punctuation_follows = char.is_ascii_punctuation();
//
//                     let left_flanking = !whitespace_follows && (
//                         !punctuation_follows || whitespace_precedes || punctuation_precedes);
//                     let right_flanking = !whitespace_precedes && (
//                         !punctuation_precedes || whitespace_follows || punctuation_follows);
//                     tokens.push(TemplateToken::Delimiter(Delimiter {
//                         character: d,
//                         length: delimiter_length,
//                         opener: left_flanking,
//                         closer: right_flanking,
//                     }));
//                     delimiter_type = None;
//                     delimiter_length = 0;
//                     start = offset - char.len_utf8();
//                 },
//                 '_' => {
//                     if char == '_' {
//                         delimiter_length += 1;
//                         continue;
//                     }
//
//                     let whitespace_follows = char.is_whitespace();
//                     let punctuation_follows = char.is_ascii_punctuation();
//
//                     let left_flanking = !whitespace_follows && (
//                         !punctuation_follows || whitespace_precedes || punctuation_precedes);
//                     let right_flanking = !whitespace_precedes && (
//                         !punctuation_precedes || whitespace_follows || punctuation_follows);
//                     tokens.push(TemplateToken::Delimiter(Delimiter {
//                         character: d,
//                         length: delimiter_length,
//                         opener: left_flanking && (!right_flanking || punctuation_precedes),
//                         closer: right_flanking && (!left_flanking || punctuation_follows),
//                     }));
//                     delimiter_type = None;
//                     delimiter_length = 0;
//                     start = offset - char.len_utf8();
//                 },
//                 _ => {},
//             }
//         }
//
//         if char.is_whitespace() {
//             if char == '\n' {
//                 newline_count += 1;
//             }
//             whitespace_precedes = true;
//             continue;
//         }
//
//         if newline_count > 1 {
//             if start < offset {
//                 tokens.push(TemplateToken::Text(String::from(WHITESPACE_REGEX.replace_all(source[start..offset - char.len_utf8()].trim_end(), " "))));
//             }
//             tokens.push(TemplateToken::ParagraphBreak);
//             start = offset - char.len_utf8();
//         }
//
//         newline_count = 0;
//
//         match char {
//             '*' => {
//                 if start < offset {
//                     tokens.push(TemplateToken::Text(String::from(WHITESPACE_REGEX.replace_all(&source[start..offset - 1], " "))));
//                 }
//                 delimiter_type = Some('*');
//                 delimiter_length = 1;
//             },
//             '_' => {
//                 if start < offset {
//                     tokens.push(TemplateToken::Text(String::from(WHITESPACE_REGEX.replace_all(&source[start..offset - 1], " "))));
//                 }
//                 delimiter_type = Some('_');
//                 delimiter_length = 1;
//             },
//             '{' => {
//                 if start < offset {
//                     tokens.push(TemplateToken::Text(String::from(WHITESPACE_REGEX.replace_all(&source[start..offset - 1], " "))));
//                 }
//                 newline_count = 0;
//                 in_tag = true;
//                 whitespace_precedes = true;
//                 start = offset;
//             },
//             _ => {
//                 whitespace_precedes = char.is_whitespace();
//                 punctuation_precedes = char.is_ascii_punctuation();
//             }
//         }
//     }
//
//     if start < source.len() {
//         tokens.push(TemplateToken::Text(String::from(WHITESPACE_REGEX.replace_all(&source[start..source.len()], " "))));
//     }
//
//     if in_tag {
//         let inner_source = &source[start..source.len()];
//         if let Some(tag) = expression_parser.parse_tag(&mut expression_lexer.lex(inner_source), attribution).parse {
//             tokens.push(TemplateToken::Tag(tag));
//         } else {
//             problems.push(Problem::fatal("Unrecognized tag", attribution));
//         }
//     } else if let Some(d) = delimiter_type {
//         match d {
//             '*' => {
//                 let whitespace_follows = true;
//                 let punctuation_follows = false;
//
//                 let left_flanking = !whitespace_follows && (
//                     !punctuation_follows || whitespace_precedes || punctuation_precedes);
//                 let right_flanking = !whitespace_precedes && (
//                     !punctuation_precedes || whitespace_follows || punctuation_follows);
//                 tokens.push(TemplateToken::Delimiter(Delimiter {
//                     character: d,
//                     length: delimiter_length,
//                     opener: left_flanking,
//                     closer: right_flanking,
//                 }));
//                 delimiter_type = None;
//                 delimiter_length = 0;
//             },
//             '_' => {
//                 let whitespace_follows = true;
//                 let punctuation_follows = false;
//
//                 let left_flanking = !whitespace_follows && (
//                     !punctuation_follows || whitespace_precedes || punctuation_precedes);
//                 let right_flanking = !whitespace_precedes && (
//                     !punctuation_precedes || whitespace_follows || punctuation_follows);
//                 tokens.push(TemplateToken::Delimiter(Delimiter {
//                     character: d,
//                     length: delimiter_length,
//                     opener: left_flanking && (!right_flanking || punctuation_precedes),
//                     closer: right_flanking && (!left_flanking || punctuation_follows),
//                 }));
//                 delimiter_type = None;
//                 delimiter_length = 0;
//             },
//             _ => {},
//         }
//     }
//
//     tokens
// }
//
// #[derive(Debug, Clone, Eq, PartialEq)]
// pub enum TemplateToken {
//     Text(String),
//     Delimiter(Delimiter),
//     ParagraphBreak,
//     Tag(TagParse),
// }
//
// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// pub struct Delimiter {
//     pub character: char,
//     pub length: usize,
//     pub opener: bool,
//     pub closer: bool,
// }
