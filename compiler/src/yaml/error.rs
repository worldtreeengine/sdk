use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use crate::yaml::mark::Mark;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SyntaxError {
        problem: &'static str,
        problem_mark: Option<Mark>,
        context: Option<&'static str>,
        context_mark: Option<Mark>,
    },
    DatasetError {
        problem: &'static str,
        problem_mark: Mark,
        context: Option<&'static str>,
        context_mark: Option<Mark>,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl StdError for Error {

}

unsafe impl Send for Error {

}

unsafe impl Sync for Error {

}

impl From<libyaml_safer::Error> for Error {
    fn from(error: libyaml_safer::Error) -> Error {
        let syntax_error = Error::SyntaxError {
            problem: error.problem(),
            problem_mark: error.problem_mark().and_then(|mark| Some(Mark::from(mark))),
            context: error.context(),
            context_mark: error.context_mark().and_then(|mark| Some(Mark::from(mark))),
        };

        if let Ok(error) =  std::io::Error::try_from(error) {
            Error::IoError(error)
        } else {
            syntax_error
        }
    }
}
