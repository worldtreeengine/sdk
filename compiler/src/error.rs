use std::error::{Error};
use std::fmt::{Debug, Display, Formatter};

pub struct SourceError {
    underlying: Box<dyn Error + Send>,
}

unsafe impl Send for SourceError {}

impl Debug for SourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.underlying, f)
    }
}

impl Display for SourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.underlying, f)
    }
}

impl Error for SourceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.underlying.source()
    }
}

impl SourceError {
    pub fn from(error: impl Error + Send + 'static) -> Self {
        SourceError {
            underlying: Box::new(error),
        }
    }
}
