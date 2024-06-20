use crate::{Attribution, Problem};
use crate::element::element::Element;
use crate::element::expression::ExpressionElement;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct ConditionalElement<E> {
    pub attribution: Attribution,
    pub when: Option<ExpressionElement>,
    pub r#if: Option<ExpressionElement>,
    pub unless: Option<ExpressionElement>,
    pub then: E,
}

impl<E: Element> Element for ConditionalElement<E> {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Mapping(map) => {
                let when = ExpressionElement::from_key(map, &attribution, "when", problems);
                let r#if = ExpressionElement::from_key(map, &attribution, "if", problems);
                let unless = ExpressionElement::from_key(map, &attribution, "unless", problems);

                if let Some(then) = E::from_key(map, &attribution, "then", problems) {
                    Self {
                        attribution,
                        when,
                        r#if,
                        unless,
                        then,
                    }
                } else {
                    let then = E::from_node(node, attribution.clone(), problems);

                    Self {
                        attribution,
                        when,
                        r#if,
                        unless,
                        then,
                    }
                }
            },
            _ => {
                let then = E::from_node(node, attribution.clone(), problems);

                Self {
                    attribution,
                    when: None,
                    r#if: None,
                    unless: None,
                    then,
                }
            }
        }
    }
}

impl<E> ConditionalElement<E> {
    pub fn always(attribution: &Attribution, then: E) -> Self {
        ConditionalElement {
            attribution: attribution.clone(),
            when: None,
            r#if: None,
            unless: None,
            then,
        }
    }
}
