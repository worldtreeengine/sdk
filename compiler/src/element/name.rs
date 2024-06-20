use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct NameElement {
    pub attribution: Attribution,
    pub name: String,
}

impl Element for NameElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(name) => NameElement { attribution, name: name.clone() },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    NameElement { attribution, name: String::new() }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single name", &attribution));
                    }

                    Self::from_node(&sequence[0], attribution.at_index(0, sequence[0].start_mark, sequence[0].end_mark), problems)
                }
            },
            Value::Mapping(mapping) => {
                if !mapping.is_empty() {
                    problems.push(Problem::fatal("Expected a name", &attribution));
                }

                NameElement { attribution, name: String::new() }
            },
        }
    }
}
