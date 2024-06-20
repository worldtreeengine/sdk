use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct TextElement {
    pub attribution: Attribution,
    pub source: String,
}

impl Element for TextElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(source) => TextElement { attribution, source: source.clone() },
            Value::Sequence(_) => {
                problems.push(Problem::fatal("Expected text, but found a sequence instead", &attribution));
                TextElement { attribution, source: String::new() }
            },
            Value::Mapping(_) => {
                problems.push(Problem::fatal("Expected text, but found a mapping instead", &attribution));
                TextElement { attribution, source: String::new() }
            },
        }
    }
}
