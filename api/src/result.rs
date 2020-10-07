use crate::result::Error::DieselError;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DieselError(diesel::result::Error),
}

impl ::std::convert::From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        DieselError(error)
    }
}