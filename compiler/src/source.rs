use std::fs::{File};
use std::path::PathBuf;
use crate::error::SourceError;
use crate::yaml::{Document, FailsafeSchema};

pub struct Source {
    pub path: String,
    pub documents: Vec<Document>,
}

impl Source {
    pub fn new(path: &str, documents: Vec<Document>) -> Source {
        Source {
            path: String::from(path),
            documents,
        }
    }

    pub fn from_string(path: &str, input: &str) -> Result<Source, SourceError> {
        FailsafeSchema::parse_string(input).map_err(|e| SourceError::from(e)).and_then(|documents| Ok(Self::new(path, documents)))
    }

    pub fn from_path(path: &PathBuf) -> Result<Source, SourceError> {
        let file = File::open(path).map_err(|e| SourceError::from(e))?;
        FailsafeSchema::parse(file).map_err(|e| SourceError::from(e)).and_then(|documents| Ok(Self::new(&path.to_string_lossy(), documents)))
    }
}

pub fn gather_sources(paths: &Vec<PathBuf>) -> Result<Vec<Source>, SourceError> {
    let mut sources: Vec<Source> = Vec::new();
    for path in paths {
        sources.push(Source::from_path(path)?);
    }

    Ok(sources)
}
