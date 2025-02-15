use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use tonlib_core::cell::{CellBuilder, CellParser};

impl ToCell for bool {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_bit(*self).map_err_to_anyhow()
    }
}

impl FromCell for bool {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_bit().map_err_to_anyhow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::bool;

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new().store_bit(true).unwrap().build().unwrap();

        assert_eq!(bool::from_cell(cell).unwrap(), true);
    }

    #[test]
    fn test_to_cell() {
        assert_eq!(
            true.to_cell().unwrap(),
            CellBuilder::new().store_bit(true).unwrap().build().unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell() {
        let first_iter = true;
        let cell = first_iter.to_cell().unwrap();
        let second_iter = bool::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
