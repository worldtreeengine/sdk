use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct LogicalValueElement {
    pub attribution: Attribution,
    pub value: bool,
}

impl Element for LogicalValueElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(value) => {
                match value.trim().to_ascii_lowercase().as_str() {
                    "yes" | "true" => LogicalValueElement { attribution, value: true },
                    "no" | "false" => LogicalValueElement { attribution, value: false },
                    _ => {
                        problems.push(Problem::fatal("Expected a logical value", &attribution));
                        LogicalValueElement { attribution, value: false }
                    }
                }
            },
            Value::Sequence(_) => {
                problems.push(Problem::fatal("Expected a logical value, but found a sequence instead", &attribution));
                LogicalValueElement { attribution, value: false }
            },
            Value::Mapping(_) => {
                problems.push(Problem::fatal("Expected a logical value, but found a mapping instead", &attribution));
                LogicalValueElement { attribution, value: false }
            },
        }
    }
}
