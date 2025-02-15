use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use tonlib_core::cell::{CellBuilder, CellParser};
use tonlib_core::message::TonMessage;

#[derive(Debug, PartialEq, Default)]
pub struct CellRef<T>(T);

impl<T> CellRef<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn inner(&self) -> &T {
        &self.0
    }
}

impl<T: ToCell> ToCell for CellRef<T> {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        let mut reference = CellBuilder::new();
        self.0.store(&mut reference)?;
        builder
            .store_reference(&reference.build()?.to_arc())
            .map_err_to_anyhow()
    }
}

impl<T: FromCell> FromCell for CellRef<T> {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        let reference = parser.next_reference()?;
        T::from_cell(reference.build()?).map(Self)
    }
}
