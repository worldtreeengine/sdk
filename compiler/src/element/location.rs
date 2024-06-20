use crate::Attribution;
use crate::element::element::Element;
use crate::element::name::NameElement;
use crate::element::named::{NamedCollectionElement, NamedElement};
use crate::element::storylet::StoryletElement;
use crate::element::template::TextTemplateElement;
use crate::problem::Problem;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct LocationElement {
    pub attribution: Attribution,
    pub name: Option<NameElement>,
    pub label: Option<TextTemplateElement>,
    pub description: Option<TextTemplateElement>,
    pub body: Option<TextTemplateElement>,
    pub storylets: Option<NamedCollectionElement<StoryletElement>>,
}

impl Element for LocationElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        Self::from_named_node(node, None, attribution, problems)
    }
}

impl NamedElement for LocationElement {
    fn from_named_node(node: &Node, name: Option<NameElement>, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(scalar) => {
                if !scalar.trim().is_empty() {
                    problems.push(Problem::fatal("Expected a storylet", &attribution));
                }

                Self {
                    attribution,
                    name,
                    label: None,
                    description: None,
                    body: None,
                    storylets: None,
                }
            },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self {
                        attribution,
                        name,
                        label: None,
                        description: None,
                        body: None,
                        storylets: None,
                    }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single storylet", &attribution));
                    }

                    Self::from_named_node(&sequence[0], name, attribution, problems)
                }
            },
            Value::Mapping(value) => {
                let name = NameElement::from_key(value, &attribution, "name", problems).or(name);
                let label = TextTemplateElement::from_key(value, &attribution, "label", problems);
                let description = TextTemplateElement::from_key(value, &attribution, "description", problems);
                let body = TextTemplateElement::from_key(value, &attribution, "body", problems);
                let storylets = NamedCollectionElement::from_key(value, &attribution, "storylets", problems);

                LocationElement {
                    attribution,
                    name,
                    label,
                    description,
                    body,
                    storylets,
                }
            }
        }
    }
}
