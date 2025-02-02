mod deserializer;
mod seq;

use crate::error::SerdeTonResult;
use serde::Deserialize;
use tonlib_core::cell::{Cell, CellParser};

pub struct CellDeserializer<'a> {
    parser: CellParser<'a>,
}

impl<'a> CellDeserializer<'a> {
    fn new(parser: CellParser<'a>) -> Self {
        Self { parser }
    }

    pub fn parse<T>(cell: &'a Cell) -> SerdeTonResult<T>
    where
        T: Deserialize<'a>,
    {
        let mut deserializer = Self::new(cell.parser());
        T::deserialize(&mut deserializer)
    }
}
