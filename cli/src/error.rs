use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct CliError {
    message: String,
}

impl Debug for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.message, f)
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl Error for CliError {

}

#[allow(dead_code)]
impl CliError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }

    pub fn from<T: Display>(value: T) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}
