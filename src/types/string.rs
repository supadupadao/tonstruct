use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use tonlib_core::cell::{CellBuilder, CellParser, MapTonCellError};

impl FromCell for String {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        let mut buf = Vec::new();
        loop {
            let remaining_bits = parser.remaining_bits();
            if remaining_bits < 8 {
                // buf.push(parser.load_u8(remaining_bits)?);
                // For some reason adds \0 char to end
                break;
            }

            if remaining_bits == 0 {
                break;
            }

            buf.push(parser.load_u8(8)?);
        }

        String::from_utf8(buf)
            .map_cell_builder_error()
            .map_err_to_anyhow()
    }
}

impl ToCell for String {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_string(self).map_err_to_anyhow()
    }
}
