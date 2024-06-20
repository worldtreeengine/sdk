mod conditional;
mod element;
mod expression;
mod list;
mod location;
mod logical;
mod meta;
mod name;
mod named;
mod numeric;
mod quality;
mod storylet;
mod text;
mod template;
mod uri;
mod tag;

use crate::{Attribution, Mark, Source};
use crate::problem::Problem;
use crate::yaml::{Document, Node, Value};

pub use crate::element::conditional::*;
pub use crate::element::element::*;
pub use crate::element::expression::*;
pub use crate::element::list::*;
pub use crate::element::location::*;
pub use crate::element::meta::*;
pub use crate::element::name::*;
pub use crate::element::named::*;
pub use crate::element::quality::*;
pub use crate::element::storylet::*;
pub use crate::element::tag::*;
pub use crate::element::text::*;
pub use crate::element::template::*;
pub use crate::element::uri::*;

#[derive(Debug)]
pub struct ElementTree {
    pub meta: Option<MetaElement>,
    pub version: Option<VersionElement>,
    pub locations: Vec<LocationElement>,
    pub qualities: Vec<QualityElement>,
    pub storylets: Vec<StoryletElement>,
}

impl ElementTree {
    pub fn from_node(node: &Node, attribution: Attribution, problems: &mut Vec<Problem>) -> Self {
        match &node.value {
            Value::Scalar(scalar) => {
                if !scalar.is_empty() {
                    problems.push(Problem::fatal("Expected document root to be a mapping", &attribution));
                }

                Self {
                    meta: None,
                    version: None,
                    locations: Vec::new(),
                    qualities: Vec::new(),
                    storylets: Vec::new(),
                }
            },
            Value::Sequence(sequence) => {
                if sequence.is_empty() {
                    Self {
                        meta: None,
                        version: None,
                        locations: Vec::new(),
                        qualities: Vec::new(),
                        storylets: Vec::new(),
                    }
                } else {
                    if sequence.len() > 1 {
                        problems.push(Problem::fatal("Expected document root to be a mapping", &attribution));
                    }

                    Self::from_node(&sequence[0], attribution.at_index(0, sequence[0].start_mark, sequence[1].end_mark), problems)
                }
            },
            Value::Mapping(mapping) => {
                let locations = if let Some(collection) = NamedCollectionElement::from_key(mapping, &attribution, "locations", problems) {
                    collection.elements
                } else {
                    Vec::new()
                };

                let qualities = if let Some(collection) = NamedCollectionElement::from_key(mapping, &attribution, "qualities", problems) {
                    collection.elements
                } else {
                    Vec::new()
                };

                let storylets = if let Some(collection) = NamedCollectionElement::from_key(mapping, &attribution, "storylets", problems) {
                    collection.elements
                } else {
                    Vec::new()
                };

                let meta = MetaElement::from_key(mapping, &attribution, "meta", problems);

                let version = VersionElement::from_key(mapping, &attribution, "version", problems);
                if let Some(version) = &version {
                    if &version.version != "0.1" {
                        problems.push(Problem::fatal("This version of the Worldtree compiler is only compatible with content version 0.1. Please upgrade your content", &version.attribution));
                    }
                }

                Self {
                    meta,
                    version,
                    locations,
                    qualities,
                    storylets,
                }
            },
        }
    }

    fn merge(self, other: Self, problems: &mut Vec<Problem>) -> Self {
        let mut locations = Vec::new();
        let mut qualities = Vec::new();
        let mut storylets = Vec::new();

        locations.extend(self.locations);
        locations.extend(other.locations);
        qualities.extend(self.qualities);
        qualities.extend(other.qualities);
        storylets.extend(self.storylets);
        storylets.extend(other.storylets);

        ElementTree {
            meta: if let Some(meta) = self.meta {
                    if let Some(other_meta) = other.meta {
                        problems.push(Problem::fatal("Metadata must only be defined in a single document", &meta.attribution).with_context("Did you intend to include only one of these?", &other_meta.attribution));
                    }
                    Some(meta)
                } else {
                    other.meta
                },
            version: if let Some(version) = self.version {
                Some(version)
            } else {
                other.version
            },
            locations,
            qualities,
            storylets,
        }
    }

    pub fn from_documents(source: &str, documents: &Vec<Document>, problems: &mut Vec<Problem>) -> Self {
        let mut tree = ElementTree {
            meta: None,
            version: None,
            qualities: Vec::new(),
            storylets: Vec::new(),
            locations: Vec::new(),
        };

        for i in 0..documents.len() {
            let document = &documents[i];
            if let Some(node) = &document.root {
                let attribution = if documents.len() == 1 {
                    Attribution::new(source, node.start_mark, node.end_mark)
                } else {
                    Attribution::new_at_index(source, i, node.start_mark, node.end_mark)
                };
                tree = tree.merge(Self::from_node(node, attribution, problems), problems);
            }
        }

        tree
    }

    pub fn from_sources(sources: &Vec<Source>, problems: &mut Vec<Problem>) -> Self {
        let mut tree = ElementTree {
            meta: None,
            version: None,
            qualities: Vec::new(),
            storylets: Vec::new(),
            locations: Vec::new(),
        };

        let mut last_source = None;

        for source in sources {
            tree = tree.merge(Self::from_documents(&source.path, &source.documents, problems), problems);
            last_source = Some(source);
        }

        if let Some(last_source) = last_source {
            if let None = tree.version {
                let attribution = Attribution::new(&last_source.path, Mark { line: 0, column: 0 }, Mark { line: 0, column: 0 });
                problems.push(Problem::fatal("At least one content source must specify a content version. Did you mean to include a version?", &attribution))
            }
        }

        tree
    }
}
