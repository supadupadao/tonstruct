use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use tonlib_core::cell::{Cell, CellBuilder, CellParser};

#[derive(Debug, PartialEq, Default)]
pub struct CellRaw(Cell);

impl From<Cell> for CellRaw {
    fn from(cell: Cell) -> Self {
        CellRaw(cell)
    }
}

impl From<CellRaw> for Cell {
    fn from(cell: CellRaw) -> Self {
        cell.0
    }
}

impl ToCell for CellRaw {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_cell_data(&self.0).map_err_to_anyhow()
    }
}

impl FromCell for CellRaw {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_remaining().map_err_to_anyhow().map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let cell = CellBuilder::default().build().unwrap();

        assert_eq!(CellRaw::from(cell.clone()), CellRaw(cell));
    }

    #[test]
    fn test_into() {
        let cell = CellBuilder::default().build().unwrap();

        assert_eq!(<CellRaw as Into<Cell>>::into(CellRaw(cell.clone())), cell);
    }

    #[test]
    fn test_default() {
        let cell = CellBuilder::default().build().unwrap();

        assert_eq!(CellRaw::default(), CellRaw(cell));
    }

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new().store_bit(true).unwrap().build().unwrap();

        assert_eq!(CellRaw::from_cell(cell.clone()).unwrap(), CellRaw(cell));
    }

    #[test]
    fn test_to_cell() {
        let cell = CellBuilder::new().store_bit(true).unwrap().build().unwrap();

        assert_eq!(CellRaw(cell.clone()).to_cell().unwrap(), cell);
    }

    #[test]
    fn test_from_to_cell() {
        let cell = CellBuilder::new().build().unwrap();
        let first_iter = CellRaw::from(cell);
        let cell = first_iter.to_cell().unwrap();
        let second_iter = CellRaw::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
