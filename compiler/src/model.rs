use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use crate::{ElementTree, Problem};
use crate::element::{AssignElement, ConditionalElement, ExpressionElement, ListElement, NameElement, StoryletElement, TextElement, TextTemplateElement, UriElement};
use crate::expression::{ExpressionAtom, ExpressionOperator, ExpressionParse, ExpressionParser};
use crate::symbol::{normalize, SymbolList};
use crate::template::{TemplateParse, TemplateParseNode, TemplateParser};
use crate::text::{Text, TextParser};

#[derive(Debug, Clone, Serialize)]
pub struct Model {
    pub meta: Meta,
    pub qualities: Vec<Quality>,
    pub locations: Vec<Location>,
    pub storylets: Vec<Storylet>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Meta {
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<Text>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<Text>,
    pub credits: Vec<Text>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Location {
    pub name: String,
    pub label: TemplateParse,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<TemplateParse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Quality {
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub singular_label: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub plural_label: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon: Option<Conditional<String>>,
    #[serde(skip_serializing_if="std::ops::Not::not")]
    pub hidden: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub style: Option<QualityStyle>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<QualityValue>>,
    #[serde(skip_serializing_if="std::ops::Not::not")]
    pub exclusive: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct QualityStyle {
    #[serde(skip_serializing_if="std::ops::Not::not")]
    pub currency: bool,
    #[serde(skip_serializing_if="std::ops::Not::not")]
    pub personal: bool,
    #[serde(skip_serializing_if="std::ops::Not::not")]
    pub plural: bool,
    #[serde(skip_serializing_if="std::ops::Not::not")]
    pub possessive: bool,
    #[serde(skip_serializing_if="std::ops::Not::not")]
    pub uncounted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct QualityValue {
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon: Option<Conditional<String>>,
}

#[derive(Debug, Clone)]
pub enum Conditional<T> {
    Always(T),
    Conditionally(ExpressionParse, T, Box<Conditional<T>>),
}

impl<T> Serialize for Conditional<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Self::Always(t) => t.serialize(serializer),
            Self::Conditionally(condition, value, next) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("condition", condition)?;
                map.serialize_entry("value", value)?;
                map.serialize_entry("next", next)?;
                map.end()
            },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Storylet {
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub condition: Option<ExpressionParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon: Option<Conditional<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub navigation: Option<Conditional<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignments: Option<Vec<AssignmentGroup>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub choices: Option<Choices>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssignmentGroup {
    pub assignments: Vec<Assignment>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<TemplateParse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Assignment {
    #[serde(skip_serializing_if="Option::is_none")]
    pub condition: Option<ExpressionParse>,
    pub subject: String,
    pub operation: AssignmentOperation,
    pub operand: ExpressionParse,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AssignmentOperation {
    Set,
    Unset,
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Serialize)]
pub struct Choices {
    #[serde(skip_serializing_if="Option::is_none")]
    pub prompt: Option<TemplateParse>,
    pub groups: Vec<ChoiceGroup>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChoiceGroup {
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<ExpressionParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub shuffle: Option<ExpressionParse>,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Choice {
    #[serde(skip_serializing_if="Option::is_none")]
    pub condition: Option<ExpressionParse>,
    pub label: TemplateParse,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon: Option<Conditional<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<TemplateParse>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub navigation: Option<Conditional<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignments: Option<Vec<AssignmentGroup>>,
}

pub struct ModelParsingResult {
    pub model: Model,
    pub problems: Vec<Problem>,
}

pub struct ModelParser {}

impl ModelParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, element_tree: &ElementTree) -> ModelParsingResult {
        let mut problems = Vec::new();
        let symbols = SymbolList::extract(element_tree, &mut problems);
        let expression_parser = ExpressionParser::new(&symbols);
        let template_parser = TemplateParser::new(&symbols);
        let text_parser = TextParser::new();
        let parse = Parse {
            expression_parser,
            template_parser,
            text_parser,
            symbols: &symbols,
            problems,
        };

        parse.parse_model(element_tree)
    }
}

struct Parse<'a> {
    template_parser: TemplateParser<'a>,
    expression_parser: ExpressionParser<'a>,
    text_parser: TextParser,
    symbols: &'a SymbolList,
    problems: Vec<Problem>,
}

impl<'a> Parse<'a> {
    fn parse_conditional_uri(&mut self, element: &Option<ListElement<ConditionalElement<UriElement>>>) -> Option<Conditional<String>> {
        if let Some(list) = element {
            let mut conditional: Option<Conditional<String>> = None;
            for list_element in list.elements.iter().rev() {
                let mut conditions = Vec::new();

                if let Some(when) = self.parse_expression(&list_element.when) {
                    conditions.push(when);
                }

                if let Some(r#if) = self.parse_expression(&list_element.r#if) {
                    conditions.push(r#if);
                }

                if let Some(unless) = self.parse_expression(&list_element.unless) {
                    conditions.push(ExpressionParse::Operation(ExpressionOperator::Not, vec!(unless)));
                }

                let condition = if conditions.is_empty() { None } else if conditions.len() == 1 { conditions.pop() } else {
                    Some(ExpressionParse::Operation(ExpressionOperator::And, conditions))
                };

                if let Some(condition) = condition {
                    if let Some(next) = conditional {
                        conditional = Some(Conditional::Conditionally(condition, list_element.then.uri.clone(), next.into()));
                    } else {
                        self.problems.push(Problem::fatal("Last option must be unconditional, but one or more conditions were provided. Did you mean to include another element?", &list.attribution));
                        conditional = Some(Conditional::Always(list_element.then.uri.clone()));
                    }
                } else {
                    conditional = Some(Conditional::Always(list_element.then.uri.clone()));
                }
            }
            conditional
        } else {
            None
        }
    }

    fn parse_conditional_name(&mut self, element: &Option<ListElement<ConditionalElement<NameElement>>>) -> Option<Conditional<String>> {
        if let Some(list) = element {
            let mut conditional: Option<Conditional<String>> = None;
            for list_element in list.elements.iter().rev() {
                let mut conditions = Vec::new();

                if let Some(when) = self.parse_expression(&list_element.when) {
                    conditions.push(when);
                }

                if let Some(r#if) = self.parse_expression(&list_element.r#if) {
                    conditions.push(r#if);
                }

                if let Some(unless) = self.parse_expression(&list_element.unless) {
                    conditions.push(ExpressionParse::Operation(ExpressionOperator::Not, vec!(unless)));
                }

                let condition = if conditions.is_empty() { None } else if conditions.len() == 1 { conditions.pop() } else {
                    Some(ExpressionParse::Operation(ExpressionOperator::And, conditions))
                };

                if let Some(condition) = condition {
                    if let Some(next) = conditional {
                        conditional = Some(Conditional::Conditionally(condition, self.symbols.require(&list_element.then, &mut self.problems), next.into()));
                    } else {
                        self.problems.push(Problem::fatal("Last option must be unconditional, but one or more conditions were provided. Did you mean to include another element?", &list.attribution));
                        conditional = Some(Conditional::Always(self.symbols.require(&list_element.then, &mut self.problems)));
                    }
                } else {
                    conditional = Some(Conditional::Always(self.symbols.require(&list_element.then, &mut self.problems)));
                }
            }
            conditional
        } else {
            None
        }
    }

    fn parse_template(&mut self, template: &Option<TextTemplateElement>) -> Option<TemplateParse> {
        if let Some(template) = template {
            let result = self.template_parser.parse(&template.source, &template.attribution);
            self.problems.extend(result.problems);
            if !result.parse.is_empty() {
                Some(result.parse)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_expression(&mut self, expression: &Option<ExpressionElement>) -> Option<ExpressionParse> {
        if let Some(expression) = expression {
            let result = self.expression_parser.parse(&expression.source, &expression.attribution);
            self.problems.extend(result.problems);
            if let Some(parse) = result.parse {
                Some(parse)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_text(&mut self, text: &Option<TextElement>) -> Option<Text> {
        if let Some(text) = text {
            let result = self.text_parser.parse(&text.source);
            self.problems.extend(result.problems);
            if !result.text.is_empty() {
                Some(result.text)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_condition(&mut self, when: &Option<ExpressionElement>, r#if: &Option<ExpressionElement>, unless: &Option<ExpressionElement>, contextual_condition: Option<ExpressionParse>) -> Option<ExpressionParse> {
        let mut conditions = Vec::new();
        conditions.extend(contextual_condition);
        conditions.extend(self.parse_expression(when));
        conditions.extend(self.parse_expression(r#if));

        if let Some(unless) = self.parse_expression(unless) {
            conditions.push(ExpressionParse::Operation(ExpressionOperator::Not, vec!(unless)));
        }

        if conditions.is_empty() {
            None
        } else {
            Some(ExpressionParse::Operation(ExpressionOperator::And, conditions))
        }
    }

    fn parse_conditional<T>(&mut self, conditional: &ConditionalElement<T>) -> Option<ExpressionParse> {
        self.parse_condition(&conditional.when, &conditional.r#if, &conditional.unless, None)
    }

    fn parse_assignments(&mut self, assignments: &Option<ListElement<AssignElement>>) -> Option<Vec<AssignmentGroup>> {
        if let Some(assignments) = assignments {
            let assignment_groups: Vec<AssignmentGroup> = assignments.elements.iter().map(|assignment_group| {
                let description = self.parse_template(&assignment_group.description);
                let assignments = assignment_group.assignments.elements.iter().map(|assignment| {
                    let condition = self.parse_conditional(assignment);

                    if let Some(set) = &assignment.then.set {
                        Some(Assignment {
                            condition,
                            subject: self.symbols.require(&set, &mut self.problems),
                            operation: AssignmentOperation::Set,
                            operand: self.parse_expression(&assignment.then.to).unwrap_or(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1))),
                        })
                    } else if let Some(unset) = &assignment.then.unset {
                        Some(Assignment {
                            condition,
                            subject: self.symbols.require(&unset, &mut self.problems),
                            operation: AssignmentOperation::Unset,
                            operand: self.parse_expression(&assignment.then.to).unwrap_or(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(0))),
                        })
                    } else if let Some(increase) = &assignment.then.increase {
                        let subject = self.symbols.require(&increase, &mut self.problems);
                        if let Some(to) = self.parse_expression(&assignment.then.to) {
                            Some(Assignment {
                                condition,
                                subject,
                                operation: AssignmentOperation::Set,
                                operand: to,
                            })
                        } else {
                            Some(Assignment {
                                condition,
                                subject,
                                operation: AssignmentOperation::Increment,
                                operand: self.parse_expression(&assignment.then.by).unwrap_or(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1))),
                            })
                        }
                    } else if let Some(decrease) = &assignment.then.decrease {
                        let subject = self.symbols.require(&decrease, &mut self.problems);
                        if let Some(to) = self.parse_expression(&assignment.then.to) {
                            Some(Assignment {
                                condition,
                                subject,
                                operation: AssignmentOperation::Unset,
                                operand: to,
                            })
                        } else {
                            Some(Assignment {
                                condition,
                                subject,
                                operation: AssignmentOperation::Decrement,
                                operand: self.parse_expression(&assignment.then.by).unwrap_or(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1))),
                            })
                        }
                    } else if let Some(increment) = &assignment.then.increment {
                        Some(Assignment {
                            condition,
                            subject: normalize(&increment.name),
                            operation: AssignmentOperation::Increment,
                            operand: self.parse_expression(&assignment.then.by).unwrap_or(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1))),
                        })
                    } else if let Some(decrement) = &assignment.then.decrement {
                        Some(Assignment {
                            condition,
                            subject: normalize(&decrement.name),
                            operation: AssignmentOperation::Decrement,
                            operand: self.parse_expression(&assignment.then.by).unwrap_or(ExpressionParse::Atom(ExpressionAtom::NumericLiteral(1))),
                        })
                    } else {
                        self.problems.push(Problem::fatal("No assignment subject or operator found", &assignment.then.attribution));
                        None
                    }
                }).flatten().collect();

                AssignmentGroup {
                    assignments,
                    description,
                }
            }).collect();

            if !assignment_groups.is_empty() {
                Some(assignment_groups)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_storylet(&mut self, storylet: &StoryletElement, contextual_condition: Option<ExpressionParse>) -> Option<Storylet> {
        if let Some(name) = &storylet.name {
            let name = name.name.clone();

            let mut conditions = Vec::new();

            if let Some(contextual_condition) = contextual_condition {
                conditions.push(contextual_condition);
            }

            if let Some(when) = self.parse_expression(&storylet.when) {
                conditions.push(when);
            }

            if let Some(r#if) = self.parse_expression(&storylet.r#if) {
                conditions.push(r#if);
            }

            if let Some(unless) = self.parse_expression(&storylet.unless) {
                conditions.push(ExpressionParse::Operation(ExpressionOperator::Not, vec!(unless)));
            }

            if !(if let Some(repeatable) = &storylet.repeatable {
                repeatable.value
            } else {
                false
            }) {
                conditions.push(ExpressionParse::Operation(ExpressionOperator::Not, vec!(ExpressionParse::Atom(ExpressionAtom::Reference(name.clone())))));
            }

            let condition = if conditions.is_empty() { None } else if conditions.len() == 1 { conditions.pop() } else {
                Some(ExpressionParse::Operation(ExpressionOperator::And, conditions))
            };

            let label = self.parse_template(&storylet.label);
            let description = self.parse_template(&storylet.description);
            let icon = self.parse_conditional_uri(&storylet.icon);
            let body = self.parse_template(&storylet.body);
            let navigation = self.parse_conditional_name(&storylet.go);

            let assignments = self.parse_assignments(&storylet.assign);

            let choices = if let Some(choose) = &storylet.choose {
                let prompt = self.parse_template(&choose.prompt);
                let groups = choose.groups.elements.iter().map(|choice_group| {
                    let limit = self.parse_expression(&choice_group.limit);
                    let shuffle = self.parse_expression(&choice_group.shuffle);
                    let choices = choice_group.choices.elements.iter().map(|choice| {
                        let condition = self.parse_conditional(&choice);

                        let label = if let Some(label) = self.parse_template(&choice.then.label) {
                            label
                        } else {
                            self.problems.push(Problem::fatal("Every choice must have a label, but this one does not. Did you mean to include a label?", &choice.then.attribution));
                            vec!(TemplateParseNode::Text("Unlabeled".to_string()))
                        };

                        let description = self.parse_template(&choice.then.description);
                        let icon = self.parse_conditional_uri(&choice.then.icon);
                        let body = self.parse_template(&choice.then.body);
                        let navigation = self.parse_conditional_name(&choice.then.go);
                        let assignments = self.parse_assignments(&choice.then.assign);

                        Choice {
                            condition,
                            label,
                            description,
                            icon,
                            body,
                            navigation,
                            assignments,
                        }
                    }).collect();

                    ChoiceGroup {
                        limit,
                        shuffle,
                        choices,
                    }
                }).collect();

                Some(Choices {
                    prompt,
                    groups,
                })
            } else {
                None
            };

            Some(Storylet {
                name,
                condition,
                label,
                description,
                icon,
                body,
                navigation,
                assignments,
                choices,
            })
        } else {
            self.problems.push(Problem::fatal("Every storylet must have a name, but this one does not. Did you mean to include a name?", &storylet.attribution));
            None
        }
    }

    fn parse_model(mut self, element_tree: &ElementTree) -> ModelParsingResult {
        let meta = if let Some(meta) = &element_tree.meta {
            let title = self.parse_text(&meta.title);
            let description = self.parse_text(&meta.description);
            let credits = if let Some(credits) = &meta.credits {
                credits.elements.iter().map(|credit| {
                    let credit = self.text_parser.parse(&credit.source);
                    self.problems.extend(credit.problems);
                    if !credit.text.is_empty() {
                        Some(credit.text)
                    } else {
                        None
                    }
                }).flatten().collect()
            } else {
                Vec::new()
            };

            Meta {
                title,
                description,
                credits,
            }
        } else {
            Meta {
                title: None,
                description: None,
                credits: Vec::new(),
            }
        };

        let qualities = element_tree.qualities.iter().map(|quality| {
            if let Some(name) = &quality.name {
                let name = name.name.clone();

                let hidden = if let Some(hidden) = &quality.hidden {
                    hidden.value
                } else {
                    false
                };

                let exclusive = if let Some(exclusive) = &quality.exclusive {
                    exclusive.value
                } else {
                    false
                };

                let values = if let Some(values) = &quality.values {
                    let values: Vec<QualityValue> = values.elements.iter().map(|value| {
                        if let Some(name) = &value.name {
                            let name = name.name.clone();
                            if hidden {
                                Some(QualityValue {
                                    name,
                                    label: None,
                                    description: None,
                                    icon: None,
                                })
                            } else {
                                let label = self.parse_template(&value.label);
                                let description = self.parse_template(&value.description);
                                let icon = self.parse_conditional_uri(&value.icon);

                                Some(QualityValue {
                                    name,
                                    label,
                                    description,
                                    icon,
                                })
                            }
                        } else {
                            self.problems.push(Problem::fatal("Every defined quality value must have a name, but this value doesn't have one. Did you mean to include a name?", &value.attribution));
                            None
                        }
                    }).flatten().collect();

                    if !values.is_empty() {
                        Some(values)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if hidden {
                    Some(Quality {
                        name,
                        hidden,
                        label: None,
                        singular_label: None,
                        plural_label: None,
                        description: None,
                        icon: None,
                        style: None,
                        values,
                        exclusive,
                    })
                } else {
                    let label = self.parse_template(&quality.label);
                    let singular_label = self.parse_template(&quality.singular_label);
                    let plural_label = self.parse_template(&quality.plural_label);
                    let description = self.parse_template(&quality.description);

                    let style = if let Some(style) = &quality.style {
                        let mut style_block = None;
                        for style_element in &style.elements {
                            match style_element.name.as_str() {
                                "currency" => {
                                    style_block = if let Some(QualityStyle { currency: _, personal, plural, possessive, uncounted }) = style_block {
                                        Some(QualityStyle { currency: true, personal, plural, possessive, uncounted })
                                    } else {
                                        Some(QualityStyle { currency: true, personal: false, plural: false, possessive: false, uncounted: false })
                                    };
                                },
                                "personal" => {
                                    style_block = if let Some(QualityStyle { currency, personal: _, plural, possessive, uncounted }) = style_block {
                                        Some(QualityStyle { currency, personal: true, plural, possessive, uncounted })
                                    } else {
                                        Some(QualityStyle { currency: false, personal: true, plural: false, possessive: false, uncounted: false })
                                    };
                                },
                                "plural" => {
                                    style_block = if let Some(QualityStyle { currency, personal, plural: _, possessive, uncounted }) = style_block {
                                        Some(QualityStyle { currency, personal, plural: true, possessive, uncounted })
                                    } else {
                                        Some(QualityStyle { currency: false, personal: false, plural: true, possessive: false, uncounted: false })
                                    };
                                },
                                "possessive" => {
                                    style_block = if let Some(QualityStyle { currency, personal, plural, possessive: _, uncounted }) = style_block {
                                        Some(QualityStyle { currency, personal, plural, possessive: true, uncounted })
                                    } else {
                                        Some(QualityStyle { currency: false, personal: false, plural: false, possessive: true, uncounted: false })
                                    };
                                },
                                "uncounted" => {
                                    style_block = if let Some(QualityStyle { currency, personal, plural, possessive, uncounted: _ }) = style_block {
                                        Some(QualityStyle { currency, personal, plural, possessive, uncounted: true })
                                    } else {
                                        Some(QualityStyle { currency: false, personal: false, plural: false, possessive: false, uncounted: true })
                                    };
                                },
                                _ => {
                                    self.problems.push(Problem::fatal("Unrecognized style tag", &style_element.attribution));
                                },
                            }
                        }
                        style_block
                    } else {
                        None
                    };

                    let icon = self.parse_conditional_uri(&quality.icon);

                    Some(Quality {
                        name,
                        label,
                        singular_label,
                        plural_label,
                        description,
                        icon,
                        hidden,
                        style,
                        values,
                        exclusive,
                    })
                }
            } else {
                self.problems.push(Problem::fatal("Every quality must have a name, but this quality doesn't have one. Did you mean to include a name?", &quality.attribution));
                None
            }
        }).flatten().collect();

        let mut storylets: Vec<Storylet> = element_tree.storylets.iter().map(|storylet| {
            self.parse_storylet(storylet, None)
        }).flatten().collect();

        let locations: Vec<Location> = element_tree.locations.iter().map(|location| {
            if let Some(name) = &location.name {
                let name = name.name.clone();
                let label = if let Some(label) = self.parse_template(&location.label) {
                    label
                } else {
                    vec!(TemplateParseNode::Text(name.clone()))
                };
                let description = self.parse_template(&location.description);
                let body = self.parse_template(&location.body);

                if let Some(local_storylets) = &location.storylets {
                    let local_storylets = local_storylets.elements.iter().map(|storylet| {
                        self.parse_storylet(storylet, Some(ExpressionParse::Operation(ExpressionOperator::In, vec!(ExpressionParse::Atom(ExpressionAtom::Reference(name.clone()))))))
                    }).flatten();
                    storylets.extend(local_storylets);
                }

                Some(Location {
                    name,
                    label,
                    description,
                    body,
                })
            } else {
                self.problems.push(Problem::fatal("Every location must have a name, but this location doesn't. Did you mean to include a name?", &location.attribution));
                None
            }
        }).flatten().collect();

        ModelParsingResult {
            model: Model {
                meta,
                qualities,
                storylets,
                locations,
            },
            problems: self.problems,
        }
    }
}
