use crate::{Attribution, Problem};
use crate::element::conditional::ConditionalElement;
use crate::element::element::Element;
use crate::element::expression::ExpressionElement;
use crate::element::list::ListElement;
use crate::element::logical::LogicalValueElement;
use crate::element::name::NameElement;
use crate::element::named::NamedElement;
use crate::element::template::TextTemplateElement;
use crate::element::uri::UriElement;
use crate::yaml::{Node, Value};

#[derive(Debug, Clone)]
pub struct StoryletElement {
    pub attribution: Attribution,
    pub name: Option<NameElement>,
    pub when: Option<ExpressionElement>,
    pub r#if: Option<ExpressionElement>,
    pub unless: Option<ExpressionElement>,
    pub repeatable: Option<LogicalValueElement>,
    pub label: Option<TextTemplateElement>,
    pub description: Option<TextTemplateElement>,
    pub icon: Option<ListElement<ConditionalElement<UriElement>>>,
    pub body: Option<TextTemplateElement>,
    pub push: Option<ListElement<ConditionalElement<ListElement<NameElement>>>>,
    pub shift: Option<ListElement<ConditionalElement<ListElement<NameElement>>>>,
    pub go: Option<ListElement<ConditionalElement<NameElement>>>,
    pub choose: Option<ChooseElement>,
    pub assign: Option<ListElement<AssignElement>>,
}

impl Element for StoryletElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        Self::from_named_node(node, None, attribution, problems)
    }
}

impl NamedElement for StoryletElement {
    fn from_named_node(node: &Node, name: Option<NameElement>, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Mapping(mapping) => {
                let name = NameElement::from_key(mapping, &attribution, "name", problems).or(name);
                let when = ExpressionElement::from_key(mapping, &attribution, "when", problems);
                let r#if = ExpressionElement::from_key(mapping, &attribution, "if", problems);
                let unless = ExpressionElement::from_key(mapping, &attribution, "unless", problems);
                let repeatable = LogicalValueElement::from_key(mapping, &attribution, "repeatable", problems);
                let label = TextTemplateElement::from_key(mapping, &attribution, "label", problems);
                let description = TextTemplateElement::from_key(mapping, &attribution, "description", problems);
                let icon = ListElement::from_key(mapping, &attribution, "icon", problems);
                let body = TextTemplateElement::from_key(mapping, &attribution, "body", problems);
                let push = ListElement::from_key(mapping, &attribution, "push", problems);
                let shift = ListElement::from_key(mapping, &attribution, "shift", problems);
                let go = ListElement::from_key(mapping, &attribution, "go", problems);
                let choose = ChooseElement::from_key(mapping, &attribution, "choose", problems);
                let assign = ListElement::from_key(mapping, &attribution, "assign", problems);

                Self {
                    attribution,
                    name,
                    when,
                    r#if,
                    unless,
                    repeatable,
                    label,
                    description,
                    icon,
                    body,
                    push,
                    shift,
                    go,
                    choose,
                    assign,
                }
            },
            Value::Scalar(scalar) => {
                if !scalar.trim().is_empty() {
                    problems.push(Problem::fatal("Expected a storylet", &attribution));
                }

                Self {
                    attribution,
                    name,
                    repeatable: None,
                    when: None,
                    r#if: None,
                    unless: None,
                    label: None,
                    description: None,
                    icon: None,
                    body: None,
                    push: None,
                    shift: None,
                    go: None,
                    choose: None,
                    assign: None,
                }
            },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self {
                        attribution,
                        name,
                        repeatable: None,
                        when: None,
                        r#if: None,
                        unless: None,
                        label: None,
                        description: None,
                        icon: None,
                        body: None,
                        push: None,
                        shift: None,
                        go: None,
                        choose: None,
                        assign: None,
                    }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single storylet", &attribution));
                    }

                    Self::from_named_node(&sequence[0], name, attribution, problems)
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChooseElement {
    pub attribution: Attribution,
    pub prompt: Option<TextTemplateElement>,

    pub groups: ListElement<ChoiceGroupElement>,
}

#[derive(Debug, Clone)]
pub struct ChoiceGroupElement {
    pub attribution: Attribution,
    pub limit: Option<ExpressionElement>,
    pub shuffle: Option<ExpressionElement>,
    pub choices: ListElement<ConditionalElement<ChoiceElement>>,
}

#[derive(Debug, Clone)]
pub struct ChoiceElement {
    pub attribution: Attribution,
    pub label: Option<TextTemplateElement>,
    pub description: Option<TextTemplateElement>,
    pub icon: Option<ListElement<ConditionalElement<UriElement>>>,
    pub body: Option<TextTemplateElement>,
    pub push: Option<ListElement<ConditionalElement<ListElement<NameElement>>>>,
    pub shift: Option<ListElement<ConditionalElement<ListElement<NameElement>>>>,
    pub go: Option<ListElement<ConditionalElement<NameElement>>>,
    pub assign: Option<ListElement<AssignElement>>,
}

impl Element for ChooseElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Mapping(map) => {
                let prompt = TextTemplateElement::from_key(map, &attribution, "prompt", problems);
                let groups = ListElement::from_key(map, &attribution, "groups", problems)
                    .unwrap_or_else(|| ListElement::from_node(node, attribution.clone(), problems));

                Self {
                    attribution,
                    prompt,
                    groups,
                }
            },
            _ => {
                let groups = ListElement::from_node(node, attribution.clone(), problems);

                Self {
                    attribution,
                    prompt: None,
                    groups,
                }
            }
        }
    }
}

impl Element for ChoiceGroupElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Mapping(map) => {
                let limit = ExpressionElement::from_key(map, &attribution, "limit", problems);
                let shuffle = ExpressionElement::from_key(map, &attribution, "shuffle", problems);
                let choices = ListElement::from_key(map, &attribution, "choices", problems)
                    .unwrap_or_else(|| ListElement::from_node(node, attribution.clone(), problems));

                Self {
                    attribution,
                    limit,
                    shuffle,
                    choices,
                }
            },
            _ => {
                let choices = ListElement::from_node(node, attribution.clone(), problems);

                Self {
                    attribution,
                    limit: None,
                    shuffle: None,
                    choices,
                }
            }
        }
    }
}

impl Element for ChoiceElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Mapping(mapping) => {
                let label = TextTemplateElement::from_key(mapping, &attribution, "label", problems);
                let description = TextTemplateElement::from_key(mapping, &attribution, "description", problems);
                let icon = ListElement::from_key(mapping, &attribution, "icon", problems);
                let body = TextTemplateElement::from_key(mapping, &attribution, "body", problems);
                let push = ListElement::from_key(mapping, &attribution, "push", problems);
                let shift = ListElement::from_key(mapping, &attribution, "shift", problems);
                let go = ListElement::from_key(mapping, &attribution, "go", problems);
                let assign = ListElement::from_key(mapping, &attribution, "assign", problems);

                Self {
                    attribution,
                    label,
                    description,
                    icon,
                    body,
                    push,
                    shift,
                    go,
                    assign,
                }
            }
            Value::Scalar(scalar) => {
                if !scalar.trim().is_empty() {
                    problems.push(Problem::fatal("Expected a choice", &attribution));
                }

                Self {
                    attribution,
                    label: None,
                    description: None,
                    icon: None,
                    body: None,
                    push: None,
                    shift: None,
                    go: None,
                    assign: None,
                }
            },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self {
                        attribution,
                        label: None,
                        description: None,
                        icon: None,
                        body: None,
                        push: None,
                        shift: None,
                        go: None,
                        assign: None,
                    }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single choice", &attribution));
                    }

                    Self::from_node(&sequence[0], attribution, problems)
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssignElement {
    pub attribution: Attribution,
    pub description: Option<TextTemplateElement>,
    pub assignments: ListElement<ConditionalElement<AssignmentElement>>,
}

#[derive(Debug, Clone)]
pub struct AssignmentElement {
    pub attribution: Attribution,
    pub set: Option<NameElement>,
    pub unset: Option<NameElement>,
    pub increase: Option<NameElement>,
    pub decrease: Option<NameElement>,
    pub increment: Option<NameElement>,
    pub decrement: Option<NameElement>,
    pub to: Option<ExpressionElement>,
    pub by: Option<ExpressionElement>,
}

impl Element for AssignElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(_) => {
                Self {
                    attribution: attribution.clone(),
                    description: None,
                    assignments: ListElement::one(&attribution, ConditionalElement::from_node(node, attribution.clone(), problems)),
                }
            },
            Value::Sequence(_) => {
                Self {
                    attribution: attribution.clone(),
                    description: None,
                    assignments: ListElement::from_node(node, attribution, problems),
                }
            },
            Value::Mapping(map) => {
                let description = TextTemplateElement::from_key(map, &attribution, "description", problems);
                let assignments = ListElement::from_key(map, &attribution, "assignments", problems);

                if let Some(assignments) = assignments {
                    Self {
                        attribution,
                        description,
                        assignments,
                    }
                } else {
                    let assignments = ListElement::from_node(node, attribution.clone(), problems);

                    Self {
                        attribution,
                        description,
                        assignments,
                    }
                }
            },
        }
    }
}

impl Element for AssignmentElement {
    fn attribution(&self) -> &Attribution {
        &self.attribution
    }

    fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(_) => {
                Self {
                    attribution: attribution.clone(),
                    set: Some(NameElement::from_node(node, attribution, problems)),
                    unset: None,
                    increase: None,
                    decrease: None,
                    increment: None,
                    decrement: None,
                    to: None,
                    by: None,
                }
            },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self {
                        attribution: attribution.clone(),
                        set: None,
                        unset: None,
                        increase: None,
                        decrease: None,
                        increment: None,
                        decrement: None,
                        to: None,
                        by: None,
                    }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected a single assignment", &attribution));
                    }

                    Self::from_node(&sequence[0], attribution, problems)
                }

            },
            Value::Mapping(map) => {
                let set = NameElement::from_key(map, &attribution, "set", problems);
                let unset = NameElement::from_key(map, &attribution, "unset", problems);
                let increase = NameElement::from_key(map, &attribution, "increase", problems);
                let decrease = NameElement::from_key(map, &attribution, "decrease", problems);
                let increment = NameElement::from_key(map, &attribution, "increment", problems);
                let decrement = NameElement::from_key(map, &attribution, "decrement", problems);
                let to = ExpressionElement::from_key(map, &attribution, "to", problems);
                let by = ExpressionElement::from_key(map, &attribution, "by", problems);

                Self {
                    attribution,
                    set,
                    unset,
                    increase,
                    decrease,
                    increment,
                    decrement,
                    to,
                    by,
                }
            },
        }
    }
}
