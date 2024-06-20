use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct UriElement {
    pub attribution: Attribution,
    pub uri: String,
}

impl Element for UriElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(uri) => UriElement { attribution, uri: String::from(uri.trim()) },
            Value::Sequence(_) => {
                problems.push(Problem::fatal("Expected a URI, but found a sequence instead", &attribution));
                UriElement { attribution, uri: String::new() }
            },
            Value::Mapping(_) => {
                problems.push(Problem::fatal("Expected a URI, but found a mapping instead", &attribution));
                UriElement { attribution, uri: String::new() }
            },
        }
    }
}
