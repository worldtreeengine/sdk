use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use libyaml_safer::{Event, EventData};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use crate::yaml::document::DocumentParsingContext;
use crate::yaml::error::Error;
use crate::yaml::mark::Mark;
use crate::yaml::path::{Path, PathElement};
use crate::yaml::schema::Schema;
use crate::yaml::value::Value;
use crate::yaml::result::Result;

#[derive(Clone, Debug, Eq)]
pub struct Node {
    pub tag: String,
    pub value: Value,
    pub start_mark: Mark,
    pub end_mark: Mark,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if match &self.value {
            Value::Scalar(_) => {
                self.tag != "tag:yaml.org,2002:str"
            },
            Value::Sequence(_) => {
                self.tag != "tag:yaml.org,2002:seq"
            },
            Value::Mapping(_) => {
                self.tag != "tag:yaml.org,2002:map"
            },
        } {
            f.write_str("!<")?;
            f.write_str(&self.tag)?;
            f.write_str("> ")?;
        }

        Display::fmt(&self.value, f)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.tag.eq(&other.tag) && self.value.eq(&other.value)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
        self.value.hash(state);
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let tag_ordering = self.tag.cmp(&other.tag);
        if let Ordering::Equal = tag_ordering {
            self.value.cmp(&other.value)
        } else {
            tag_ordering
        }
    }
}

const STRING_TAG: &str = "tag:yaml.org,2002:str";

impl Node {
    pub fn string(string: &str) -> Node {
        Node {
            tag: String::from(STRING_TAG),
            value: Value::Scalar(String::from(string)),
            start_mark: Mark::default(),
            end_mark: Mark::default(),
        }
    }
}

pub struct NodeParsingContext<'p, 'd, 'n, S: Schema> {
    document: &'n mut DocumentParsingContext<'p, 'd, S>,
    path: Path<'n>,
}

impl<'p, 'd, 'n, S: Schema> NodeParsingContext<'p, 'd, 'n, S> {
    pub fn new(document: &'n mut DocumentParsingContext<'p, 'd, S>) -> NodeParsingContext<'p, 'd, 'n, S> {
        NodeParsingContext {
            document,
            path: Path::new(),
        }
    }

    fn next(&mut self) -> Result<Event> {
        self.document.next()
    }

    fn resolve(&self, tag: Option<String>, value: Value, start_mark: Mark, end_mark: Mark) -> Node {
        match tag {
            None => {
                let (tag, value) = self.document.resolve(&self.path, value);
                Node {
                    tag,
                    value,
                    start_mark,
                    end_mark,
                }
            },
            Some(tag) => {
                let tag = self.document.resolve_tag(tag);

                Node {
                    tag,
                    value,
                    start_mark,
                    end_mark,
                }
            }
        }
    }

    fn with_path_element<'x>(&'x mut self, element: PathElement<'x>) -> NodeParsingContext<'p, 'd, 'x, S> {
        let mut path = self.path.clone();
        path.push(element);

        NodeParsingContext {
            document: &mut self.document,
            path,
        }
    }

    fn resolve_alias(&self, anchor: &str) -> Option<Node> {
        self.document.resolve_alias(anchor)
    }

    fn define_anchor(&mut self, anchor: String, node: Node) {
        self.document.define_anchor(anchor, node);
    }

    fn parse_mapping(&mut self, tag: Option<String>, start_mark: Mark) -> Result<Node> {
        let mut map = BTreeMap::new();

        loop {
            let key_event = self.next()?;

            if let EventData::MappingEnd { .. } = key_event.data {
                return Ok(self.resolve(tag, Value::Mapping(map), start_mark, key_event.end_mark.into()));
            }

            let key = self.parse(key_event)?;

            if let Some((k, _)) = map.get_key_value(&key) {
                return Err(Error::DatasetError {
                    problem: "Mapping contains duplicate key",
                    problem_mark: key.start_mark,
                    context: Some("Already defined"),
                    context_mark: Some(k.start_mark),
                });
            }

            let value_event = self.next()?;
            let value = self.with_path_element(PathElement::Key(&key)).parse(value_event)?;
            map.insert(key, value);
        }
    }

    fn parse_sequence(&mut self, tag: Option<String>, start_mark: Mark) -> Result<Node> {
        let mut vector = Vec::new();

        loop {
            let element_event = self.next()?;

            if let EventData::SequenceEnd { .. } = element_event.data {
                return Ok(self.resolve(tag, Value::Sequence(vector), start_mark, element_event.end_mark.into()));
            }

            ;
            vector.push(self.with_path_element(PathElement::Index(vector.len())).parse(element_event)?);
        }
    }

    pub fn parse(&mut self, event: Event) -> Result<Node> {
        match event.data {
            EventData::Scalar { anchor, tag, value, .. } => {
                let node = self.resolve(tag, Value::Scalar(value), event.start_mark.into(), event.end_mark.into());
                if let Some(anchor) = anchor {
                    self.define_anchor(anchor, node.clone());
                }
                Ok(node)
            },
            EventData::SequenceStart { anchor, tag, .. } => {
                let sequence = self.parse_sequence(tag, event.start_mark.into())?;
                if let Some(anchor) = anchor {
                    self.define_anchor(anchor, sequence.clone());
                }
                Ok(sequence)
            },
            EventData::MappingStart { anchor, tag, .. } => {
                let mapping = self.parse_mapping(tag, event.start_mark.into())?;
                if let Some(anchor) = anchor {
                    self.define_anchor(anchor, mapping.clone());
                }
                Ok(mapping)
            },
            EventData::Alias { anchor } => {
                if let Some(node) = self.resolve_alias(&anchor) {
                    Ok(node)
                } else {
                    return Err(Error::DatasetError {
                        problem: "Undefined anchor",
                        problem_mark: event.start_mark.into(),
                        context: None, context_mark: None,
                    });
                }
            },
            _ => {
                return Err(Error::DatasetError {
                    problem: "Expected a node",
                    problem_mark: event.start_mark.into(),
                    context: None, context_mark: None,
                });
            },
        }
    }
}
