use libyaml_safer::{Event, EventData, Parser, TagDirective};
use crate::yaml::document::{Document, DocumentParsingContext};
use crate::yaml::error::Error;
use crate::yaml::{Path, Value};
use crate::yaml::result::{Result, ToResult};
use crate::yaml::schema::Schema;

pub struct ParsingContext<'p, S: Schema> {
    parser: &'p mut Parser<'p>,
    schema: &'p S,
}

impl<'p, S: Schema> ParsingContext<'p, S> {
    pub fn new(parser: &'p mut Parser<'p>, schema: &'p S) -> ParsingContext<'p, S> {
        ParsingContext {
            parser,
            schema,
        }
    }

    pub fn next(&mut self) -> Result<Event> {
        self.parser.parse().to_result()
    }

    pub fn document<'d>(&'d mut self, tag_directives: Vec<TagDirective>) -> DocumentParsingContext<'p, 'd, S> {
        DocumentParsingContext::new(self, tag_directives)
    }

    pub fn resolve(&self, path: &Path, value: Value) -> (String, Value) {
        self.schema.resolve(path, value)
    }

    pub fn parse(&'p mut self) -> Result<Vec<Document>> {
        let mut documents: Vec<Document> = Vec::new();

        loop {
            let event = self.next()?;
            match event.data {
                EventData::StreamStart { .. } => continue,
                EventData::DocumentStart { tag_directives, .. } => {
                    documents.push(self.document(tag_directives).parse(event.start_mark)?)
                },
                EventData::StreamEnd { .. } => break,
                _ => {
                    return Err(Error::DatasetError {
                        problem: "Expected a document",
                        problem_mark: crate::yaml::Mark::from(event.start_mark),
                        context: None, context_mark: None,
                    });
                },
            };
        }

        return Ok(documents);
    }
}
