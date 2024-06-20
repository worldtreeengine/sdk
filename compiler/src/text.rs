mod lexer;
mod parse;

pub type Text = Vec<TextNode>;

#[derive(Debug, Clone)]
pub enum TextNode {
    Plain(String),
    Paragraph(Text),
    Italic(Text),
    Bold(Text),
    Anchor(String, Text),
}

impl Serialize for TextNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Self::Plain(s) => serializer.serialize_str(s),
            Self::Paragraph(p) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("p", p)?;
                map.end()
            },
            Self::Italic(i) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("i", i)?;
                map.end()
            },
            Self::Bold(b) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("b", b)?;
                map.end()
            },
            Self::Anchor(href, a) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("a", a)?;
                map.serialize_entry("href", href)?;
                map.end()
            },
        }
    }
}

use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
pub use crate::text::parse::*;
