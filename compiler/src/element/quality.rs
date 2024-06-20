use crate::Attribution;
use crate::element::element::Element;
use crate::element::named::{NamedCollectionElement, NamedElement};
use crate::element::logical::LogicalValueElement;
use crate::element::name::NameElement;
use crate::element::{ConditionalElement, ListElement, TagElement, UriElement};
use crate::element::template::TextTemplateElement;
use crate::problem::Problem;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct QualityElement {
    pub attribution: Attribution,
    pub name: Option<NameElement>,
    pub hidden: Option<LogicalValueElement>,
    pub label: Option<TextTemplateElement>,
    pub singular_label: Option<TextTemplateElement>,
    pub plural_label: Option<TextTemplateElement>,
    pub description: Option<TextTemplateElement>,
    pub style: Option<ListElement<TagElement>>,
    pub icon: Option<ListElement<ConditionalElement<UriElement>>>,
    pub values: Option<NamedCollectionElement<QualityValueElement>>,
    pub exclusive: Option<LogicalValueElement>,
}

impl Element for QualityElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        Self::from_named_node(node, None, attribution, problems)
    }
}

impl NamedElement for QualityElement {
    fn from_named_node(node: &Node, name: Option<NameElement>, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(scalar) => {
                if !scalar.is_empty() {
                    problems.push(Problem::fatal("Expected a quality", &attribution));
                }

                Self {
                    attribution,
                    name,
                    hidden: None,
                    label: None,
                    singular_label: None,
                    plural_label: None,
                    description: None,
                    values: None,
                    style: None,
                    exclusive: None,
                    icon: None,
                }
            },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self {
                        attribution,
                        name,
                        hidden: None,
                        label: None,
                        singular_label: None,
                        plural_label: None,
                        description: None,
                        values: None,
                        style: None,
                        exclusive: None,
                        icon: None,
                    }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single quality", &attribution));
                    }

                    Self::from_named_node(&sequence[0], name, attribution.at_index(0, sequence[0].start_mark, sequence[0].end_mark), problems)
                }
            },
            Value::Mapping(value) => {
                let name = NameElement::from_key(value, &attribution, "name", problems).or(name);
                let hidden = LogicalValueElement::from_key(value, &attribution, "hidden", problems);
                let label = TextTemplateElement::from_key(value, &attribution, "label", problems);
                let singular_label = TextTemplateElement::from_key(value, &attribution, "singularLabel", problems);
                let plural_label = TextTemplateElement::from_key(value, &attribution, "pluralLabel", problems);
                let description = TextTemplateElement::from_key(value, &attribution, "description", problems);
                let values = NamedCollectionElement::from_key(value, &attribution, "values", problems);
                let style = ListElement::from_key(value, &attribution, "style", problems);
                let exclusive = LogicalValueElement::from_key(value, &attribution, "exclusive", problems);
                let icon = ListElement::from_key(value, &attribution, "icon", problems);

                Self {
                    attribution,
                    name,
                    hidden,
                    label,
                    singular_label,
                    plural_label,
                    description,
                    values,
                    style,
                    exclusive,
                    icon,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct QualityValueElement {
    pub attribution: Attribution,
    pub name: Option<NameElement>,
    pub label: Option<TextTemplateElement>,
    pub description: Option<TextTemplateElement>,
    pub style: Option<ListElement<TagElement>>,
    pub icon: Option<ListElement<ConditionalElement<UriElement>>>,
}

impl Element for QualityValueElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        Self::from_named_node(node, None, attribution, problems)
    }
}

impl NamedElement for QualityValueElement {
    fn from_named_node(node: &Node, name: Option<NameElement>, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(scalar) => {
                if !scalar.is_empty() {
                    problems.push(Problem::fatal("Expected a quality value", &attribution));
                }

                Self {
                    attribution,
                    name,
                    label: None,
                    description: None,
                    style: None,
                    icon: None,
                }
            },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self {
                        attribution,
                        name,
                        label: None,
                        description: None,
                        style: None,
                        icon: None,
                    }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single quality value", &attribution));
                    }

                    Self::from_named_node(&sequence[0], name, attribution.at_index(0, sequence[0].start_mark, sequence[0].end_mark), problems)
                }
            },
            Value::Mapping(value) => {
                let name = NameElement::from_key(value, &attribution, "name", problems).or(name);
                let label = TextTemplateElement::from_key(value, &attribution, "label", problems);
                let description = TextTemplateElement::from_key(value, &attribution, "description", problems);
                let style = ListElement::from_key(value, &attribution, "style", problems);
                let icon = ListElement::from_key(value, &attribution, "icon", problems);

                Self {
                    attribution,
                    name,
                    label,
                    description,
                    style,
                    icon,
                }
            }
        }
    }
}
