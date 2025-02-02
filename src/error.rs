use serde::{de, ser};
use std::fmt::{Display, Formatter};

pub type SerdeTonResult<T, E = SerdeTonError> = Result<T, E>;

#[derive(Debug)]
pub enum SerdeTonError {
    CellBuild(String),
    FieldError(String),
}

impl ser::Error for SerdeTonError {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

impl de::Error for SerdeTonError {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

impl Display for SerdeTonError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for SerdeTonError {}
