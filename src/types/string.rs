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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new()
            .store_slice("Hello world".as_bytes())
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(String::from_cell(cell).unwrap(), "Hello world".to_string());
    }

    #[test]
    fn test_to_cell() {
        assert_eq!(
            "Hello world".to_string().to_cell().unwrap(),
            CellBuilder::new()
                .store_slice("Hello world".as_bytes())
                .unwrap()
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell() {
        let first_iter = "Hello world".to_string();
        let cell = first_iter.to_cell().unwrap();
        let second_iter = String::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
