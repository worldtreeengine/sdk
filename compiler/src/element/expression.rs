use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct ExpressionElement {
    pub attribution: Attribution,
    pub source: String,
}

impl Element for ExpressionElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(source) => ExpressionElement { attribution, source: source.clone() },
            Value::Sequence(_) => {
                problems.push(Problem::fatal("Expected an expression, but found a sequence instead", &attribution));
                ExpressionElement { attribution, source: String::from("no") }
            },
            Value::Mapping(_) => {
                problems.push(Problem::fatal("Expected an expression, but found a mapping instead", &attribution));
                ExpressionElement { attribution, source: String::from("no") }
            },
        }
    }
}
