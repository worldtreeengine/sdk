use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::yaml::node::Node;

#[derive(Clone, Debug, Eq)]
pub enum Value {
    Scalar(String),
    Sequence(Vec<Node>),
    Mapping(BTreeMap<Node, Node>),
}

impl Value {
    pub fn as_scalar(&self) -> Option<&String> {
        if let Self::Scalar(string) = self {
            Some(string)
        } else {
            None
        }
    }

    pub fn as_sequence(&self) -> Option<&Vec<Node>> {
        if let Self::Sequence(vec) = self {
            Some(vec)
        } else {
            None
        }
    }

    pub fn as_mapping(&self) -> Option<&BTreeMap<Node, Node>> {
        if let Self::Mapping(btree) = self {
            Some(btree)
        } else {
            None
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(string) => {
                f.write_str(string)
            },
            Self::Sequence(vec) => {
                f.write_str("[")?;
                let mut first = true;
                for node in vec {
                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;
                    Display::fmt(node, f)?;
                }
                f.write_str("]")
            },
            Self::Mapping(btree) => {
                f.write_str("{")?;
                let mut first = true;
                for (key, value) in btree {
                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;
                    Display::fmt(key, f)?;
                    f.write_str(" = ")?;
                    Display::fmt(value, f)?;
                }
                f.write_str("]")
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Scalar(value) => {
                if let Self::Scalar(other_value) = other {
                    value.eq(other_value)
                } else {
                    false
                }
            },
            Self::Sequence(value) => {
                if let Self::Sequence(other_value) = other {
                    value.eq(other_value)
                } else {
                    false
                }
            },
            Self::Mapping(value) => {
                if let Self::Mapping(other_value) = other {
                    value.eq(other_value)
                } else {
                    false
                }
            },
        }
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Scalar(scalar) => scalar.hash(state),
            Self::Sequence(sequence) => sequence.hash(state),
            Self::Mapping(mapping) => mapping.hash(state),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Scalar(scalar) =>
                match other {
                    Self::Scalar(other_scalar) => scalar.cmp(other_scalar),
                    _ => Ordering::Greater,
                },
            Self::Sequence(sequence) =>
                match other {
                    Self::Scalar(_) => Ordering::Less,
                    Self::Sequence(other_sequence) => sequence.cmp(other_sequence),
                    _ => Ordering::Greater,
                },
            Self::Mapping(mapping) =>
                match other {
                    Self::Mapping(other_mapping) => mapping.cmp(other_mapping),
                    _ => Ordering::Less,
                },
        }
    }
}
