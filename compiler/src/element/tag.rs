use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct TagElement {
    pub attribution: Attribution,
    pub name: String,
}

impl Element for TagElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(name) => TagElement { attribution, name: name.trim().to_lowercase() },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    TagElement { attribution, name: String::new() }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single tag", &attribution));
                    }

                    Self::from_node(&sequence[0], attribution.at_index(0, sequence[0].start_mark, sequence[0].end_mark), problems)
                }
            },
            Value::Mapping(mapping) => {
                if !mapping.is_empty() {
                    problems.push(Problem::fatal("Expected a tag", &attribution));
                }

                TagElement { attribution, name: String::new() }
            },
        }
    }
}
