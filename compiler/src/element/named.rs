use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::element::name::NameElement;
use crate::yaml::{Node, Value};

pub trait NamedElement: Element {
    fn from_named_node(node: &Node, name: Option<NameElement>, attribution: Attribution, problems: &mut Vec<Problem>) -> Self;
}

#[derive(Debug, Clone)]
pub struct NamedCollectionElement<E: NamedElement> {
    pub attribution: Attribution,
    pub elements: Vec<E>,
}

impl<E: NamedElement> Element for NamedCollectionElement<E> {
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
            Value::Mapping(mapping) => {
                let mut elements = Vec::new();
                let mut i = 0usize;
                for (key, value) in mapping {
                    let key_attribution = attribution.at_index(i, key.start_mark, key.end_mark);
                    let name = NameElement::from_node(key, key_attribution, problems);
                    let value_attribution = attribution.at_key(&name.name, value.start_mark, value.end_mark);
                    elements.push(E::from_named_node(value, Some(name), value_attribution, problems));
                    i += 1;
                };
                Self {
                    attribution,
                    elements,
                }
            }
            Value::Scalar(_) => {
                let element = E::from_node(&node, attribution.clone(), problems);
                Self { attribution, elements: vec!(element) }
            },
        }
    }
}
