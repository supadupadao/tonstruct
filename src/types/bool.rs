use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use tonlib_core::cell::{CellBuilder, CellParser};

#[derive(Debug, PartialEq, Default)]
pub struct Bool(bool);

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Bool(value)
    }
}

impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        value.0
    }
}

impl ToCell for Bool {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_bit(self.0).map_err_to_anyhow()
    }
}

impl FromCell for Bool {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_bit().map(Bool).map_err_to_anyhow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Bool::from(true), Bool(true));
    }

    #[test]
    fn test_into() {
        assert_eq!(<Bool as Into<bool>>::into(Bool(true)), true);
    }

    #[test]
    fn test_default() {
        assert_eq!(Bool::default(), Bool(false));
    }

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new().store_bit(true).unwrap().build().unwrap();

        assert_eq!(Bool::from_cell(cell).unwrap(), Bool(true));
    }

    #[test]
    fn test_to_cell() {
        assert_eq!(
            Bool(true).to_cell().unwrap(),
            CellBuilder::new().store_bit(true).unwrap().build().unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell() {
        let first_iter = Bool::from(true);
        let cell = first_iter.to_cell().unwrap();
        let second_iter = Bool::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
