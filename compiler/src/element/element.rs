use std::collections::BTreeMap;
use crate::{Attribution, Problem};
use crate::yaml::Node;

pub trait Element {
    fn attribution(&self) -> &Attribution;
    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self;

    fn from_key(mapping: &BTreeMap<Node, Node>, attribution: &Attribution, key: &str, problems: &mut Vec<Problem>) -> Option<Self> where Self: Sized {
        if let Some(node) = mapping.get(&Node::string(key)) {
            let node_attribution = attribution.at_key(key, node.start_mark, node.end_mark);
            Some(Self::from_node(node, node_attribution, problems))
        } else {
            None
        }
    }

    fn from_sequence(sequence: &Vec<Node>, attribution: &Attribution, problems: &mut Vec<Problem>) -> Vec<Self> where Self: Sized {
        let mut elements: Vec<Self> = Vec::new();
        for i in 0..sequence.len() {
            let element_node = &sequence[i];
            let element_attribution = attribution.at_index(i, element_node.start_mark, element_node.end_mark);
            elements.push(Self::from_node(element_node, element_attribution, problems));
        }
        elements
    }
}
