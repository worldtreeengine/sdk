use crate::yaml::error::Error;

pub trait ToResult<T> {
    fn to_result(self) -> Result<T>;
}

impl<T> ToResult<T> for std::result::Result<T, libyaml_safer::Error> {
    fn to_result(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(Error::from(error)),
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
