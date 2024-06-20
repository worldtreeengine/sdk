use libyaml_safer::{Event, EventData, Mark, TagDirective};
use std::collections::HashMap;
use crate::yaml::error::Error;
use crate::yaml::node::{Node, NodeParsingContext};
use crate::yaml::{Path, result, Value};
use crate::yaml::context::ParsingContext;
use crate::yaml::schema::Schema;

#[derive(PartialEq, Debug)]
pub struct Document {
    pub root: Option<Node>,
    pub start_mark: Mark,
    pub end_mark: Mark,
}

pub struct DocumentParsingContext<'p, 'd, S: Schema> {
    context: &'d mut ParsingContext<'p, S>,
    primary_tag_directive: Option<TagDirective>,
    non_primary_tag_directives: Vec<TagDirective>,
    anchors: HashMap<String, Node>,
}

impl<'p, 'd, S: Schema> DocumentParsingContext<'p, 'd, S> {
    pub fn new(context: &'d mut ParsingContext<'p, S>, tag_directives: Vec<TagDirective>) -> DocumentParsingContext<'p, 'd, S> {
        let mut primary_tag_directive: Option<TagDirective> = None;
        let mut non_primary_tag_directives = Vec::new();
        for tag_directive in tag_directives {
            if tag_directive.handle == "!" {
                primary_tag_directive = Some(tag_directive);
            } else {
                non_primary_tag_directives.push(tag_directive);
            }
        }

        DocumentParsingContext {
            context,
            primary_tag_directive,
            non_primary_tag_directives,
            anchors: HashMap::new(),
        }
    }

    pub fn next(&mut self) -> result::Result<Event> {
        self.context.next()
    }

    pub fn node<'n>(&'n mut self) -> NodeParsingContext<'p, 'd, 'n, S> {
        NodeParsingContext::new(self)
    }

    pub fn resolve(&self, path: &Path, value: Value) -> (String, Value) {
        self.context.resolve(path, value)
    }

    pub fn resolve_tag(&self, tag: String) -> String {
        if tag.starts_with("!<") {
            String::from(&tag[2..tag.len() - 1])
        } else if let Some(tag_directive) = self.non_primary_tag_directives.iter().find(|tag_directive| tag.starts_with(&tag_directive.handle)) {
            format!("{}{}", tag_directive.prefix, &tag[tag_directive.handle.len()..])
        } else if tag.starts_with("!!") {
            format!("tag:yaml.org,2002:{}", &tag[2..])
        } else if let Some(primary_tag_directive) = &self.primary_tag_directive {
            format!("{}{}", primary_tag_directive.prefix, &tag[1..])
        } else {
            tag
        }
    }

    pub fn resolve_alias(&self, anchor: &str) -> Option<Node> {
        if let Some(node) = self.anchors.get(anchor) {
            Some(node.clone().clone())
        } else {
            None
        }
    }

    pub fn define_anchor(&mut self, anchor: String, node: Node) {
        self.anchors.insert(anchor, node);
    }

    pub fn parse(mut self, start_mark: Mark) -> result::Result<Document> {
        let next_event = self.next()?;
        let root = match &next_event.data {
            EventData::DocumentEnd { .. } => return Ok(Document {
                root: None,
                start_mark,
                end_mark: next_event.end_mark,
            }),
            _ => {
                self.node().parse(next_event)?
            }
        };
        let last_event = self.next()?;
        if let EventData::DocumentEnd { .. } = last_event.data {
            Ok(Document {
                root: Some(root),
                start_mark,
                end_mark: last_event.end_mark,
            })
        } else {
            Err(Error::DatasetError {
                problem: "Expected end of document",
                problem_mark: crate::yaml::Mark::from(last_event.start_mark),
                context: None, context_mark: None,
            })
        }
    }
}
