mod attribution;
mod error;
mod source;
mod element;
mod model;
mod problem;
mod yaml;
mod expression;
mod symbol;
mod r#template;
mod string_table;
mod text;

use std::path::PathBuf;
pub use attribution::Attribution;
pub use yaml::Mark;
use source::{gather_sources, Source};
use element::ElementTree;
pub use problem::{Context, Problem};
pub use error::SourceError;
pub use model::*;
pub use template::*;
pub use text::*;
pub use expression::*;

pub fn compile(paths: &Vec<PathBuf>) -> Result<ModelParsingResult, SourceError> {
    let sources = gather_sources(paths)?;
    let mut problems = Vec::new();
    let tree = ElementTree::from_sources(&sources, &mut problems);
    let model = ModelParser::new().parse(&tree);
    problems.extend(model.problems);
    Ok(ModelParsingResult { model: model.model, problems })
}
