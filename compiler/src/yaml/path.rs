use std::fmt::{Display, Formatter, Write};
use crate::yaml::node::Node;

#[derive(Clone)]
pub enum PathElement<'r> {
    Key(&'r Node),
    Index(usize),
}

impl<'r> Display for PathElement<'r> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(node) => {
                f.write_char('.')?;
                node.fmt(f)
            },
            Self::Index(index) => {
                f.write_char('[')?;
                Display::fmt(index, f)?;
                f.write_char(']')
            }
        }
    }
}

pub struct Path<'r> {
    vec: Vec<PathElement<'r>>,
}

impl<'r> Path<'r> {
    pub fn new() -> Path<'r> {
        Path {
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, element: PathElement<'r>) {
        self.vec.push(element);
    }
}

impl<'r> Clone for Path<'r> {
    fn clone(&self) -> Self {
        Path {
            vec: self.vec.clone(),
        }
    }
}

impl<'r> Display for Path<'r> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for element in &self.vec {
            Display::fmt(element, f)?;
        }
        Ok(())
    }
}
