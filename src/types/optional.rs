use crate::{FromCell, ToCell};
use tonlib_core::cell::{CellBuilder, CellParser};

impl<T> ToCell for Option<T>
where
    T: ToCell,
{
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_bit(self.is_some())?;
        if let Some(inner) = self {
            inner.store(builder)?;
        }
        Ok(builder)
    }
}

impl<T> FromCell for Option<T>
where
    T: FromCell,
{
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        match parser.load_bit()? {
            true => T::load(parser).map(Some),
            false => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(FromCell, ToCell, Debug, PartialEq, Default)]
    struct Message {
        optional_bool: Option<bool>,
    }

    #[test]
    fn test_from_cell_some() {
        let cell = CellBuilder::new()
            .store_bit(true)
            .unwrap()
            .store_bit(true)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            Message::from_cell(cell).unwrap(),
            Message {
                optional_bool: Some(true)
            }
        );
    }

    #[test]
    fn test_from_cell_none() {
        let cell = CellBuilder::new()
            .store_bit(false)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            Message::from_cell(cell).unwrap(),
            Message {
                optional_bool: None
            }
        );
    }

    #[test]
    fn test_to_cell_some() {
        assert_eq!(
            Message {
                optional_bool: Some(true)
            }
            .to_cell()
            .unwrap(),
            CellBuilder::new()
                .store_bit(true)
                .unwrap()
                .store_bit(true)
                .unwrap()
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn test_to_cell_none() {
        assert_eq!(
            Message {
                optional_bool: None
            }
            .to_cell()
            .unwrap(),
            CellBuilder::new()
                .store_bit(false)
                .unwrap()
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell_some() {
        let first_iter = Message {
            optional_bool: Some(true),
        };
        let cell = first_iter.to_cell().unwrap();
        let second_iter = Message::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }

    #[test]
    fn test_from_to_cell_none() {
        let first_iter = Message {
            optional_bool: None,
        };
        let cell = first_iter.to_cell().unwrap();
        let second_iter = Message::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
