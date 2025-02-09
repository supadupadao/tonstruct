use tonlib_core::cell::{Cell, CellParser};

pub trait FromCell: Sized {
    fn from_cell(cell: Cell) -> anyhow::Result<Self> {
        let mut parser = cell.parser();
        Self::load(&mut parser)
    }

    fn load(parser: &mut CellParser) -> anyhow::Result<Self>;
}
