use crate::{Attribution, Problem};
use crate::element::element::{Element};
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct ListElement<E> {
    pub attribution: Attribution,
    pub elements: Vec<E>,
}

impl<E: Element> Element for ListElement<E> {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Sequence(elements) => {
                let elements = E::from_sequence(elements, &attribution, problems);
                Self {
                    attribution,
                    elements,
                }
            },
            _ => {
                let element = E::from_node(&node, attribution.clone(), problems);
                Self { attribution, elements: vec!(element) }
            },
        }
    }
}

impl<E> ListElement<E> {
    pub fn one(attribution: &Attribution, element: E) -> Self {
        Self {
            attribution: attribution.clone(),
            elements: vec!(element),
        }
    }

    pub fn one_or_none(attribution: &Attribution, element: Option<E>) -> Self {
        if let Some(element) = element {
            Self::one(attribution, element)
        } else {
            Self {
                attribution: attribution.clone(),
                elements: vec!(),
            }
        }
    }
}
