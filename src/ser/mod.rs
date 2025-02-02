mod map;
mod seq;
mod serializer;
mod r#struct;
mod struct_variant;
mod tuple;
mod tuple_struct;
mod tuple_variant;

use crate::error::{SerdeTonError, SerdeTonResult};
use serde::Serialize;
use tonlib_core::cell::{Cell, CellBuilder};

pub struct CellSerializer {
    builder: CellBuilder,
}

impl CellSerializer {
    fn new() -> CellSerializer {
        CellSerializer {
            builder: CellBuilder::new(),
        }
    }

    pub fn to_cell<T>(value: T) -> SerdeTonResult<Cell>
    where
        T: Serialize,
    {
        let mut serializer = Self::new();
        value.serialize(&mut serializer)?;
        serializer
            .builder
            .build()
            .map_err(|err| SerdeTonError::CellBuild(err.to_string()))
    }
}
