use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::element::list::ListElement;
use crate::element::text::TextElement;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct MetaElement {
    pub attribution: Attribution,
    pub title: Option<TextElement>,
    pub description: Option<TextElement>,
    pub credits: Option<ListElement<TextElement>>,
}

#[derive(Debug, Clone)]
pub struct VersionElement {
    pub attribution: Attribution,
    pub version: String,
}

impl Element for VersionElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(version) => Self { attribution, version: String::from(version.trim()) },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self { attribution, version: String::new() }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single version", &attribution));
                    }

                    Self::from_node(&sequence[0], attribution.at_index(0, sequence[0].start_mark, sequence[0].end_mark), problems)
                }
            },
            Value::Mapping(_) => {
                problems.push(Problem::fatal("Expected a version, but found a mapping instead", &attribution));
                Self { attribution, version: String::new() }
            },
        }
    }
}

impl Element for MetaElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(scalar) => {
                if !scalar.trim().is_empty() {
                    problems.push(Problem::fatal("Expected a metadata block", &attribution));
                }

                Self {
                    attribution,
                    title: None,
                    description: None,
                    credits: None,
                }
            },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self {
                        attribution,
                        title: None,
                        description: None,
                        credits: None,
                    }
                } else {
                    if sequence.len() > 0 {
                        problems.push(Problem::fatal("Expected a single metadata block", &attribution));
                    }

                    Self::from_node(&sequence[0], attribution.at_index(0, sequence[0].start_mark, sequence[0].end_mark), problems)
                }
            },
            Value::Mapping(map) => {
                let title = TextElement::from_key(map, &attribution, "title", problems);
                let description = TextElement::from_key(map, &attribution, "description", problems);
                let credits = ListElement::from_key(map, &attribution, "credits", problems);

                Self {
                    attribution,
                    title,
                    description,
                    credits,
                }
            }
        }
    }
}
