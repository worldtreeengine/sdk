use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct NumericValueElement {
    pub attribution: Attribution,
    pub value: u32,
}

impl Element for NumericValueElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(value) => {
                if let Ok(value) = u32::from_str_radix(value.trim(), 10) {
                    NumericValueElement { attribution, value }
                } else {
                    problems.push(Problem::fatal("Expected a numeric value", &attribution));
                    NumericValueElement { attribution, value: 0 }
                }
            },
            Value::Sequence(_) => {
                problems.push(Problem::fatal("Expected a numeric value, but found a sequence instead", &attribution));
                NumericValueElement { attribution, value: 0 }
            },
            Value::Mapping(_) => {
                problems.push(Problem::fatal("Expected a numeric value, but found a mapping instead", &attribution));
                NumericValueElement { attribution, value: 0 }
            },
        }
    }
}
