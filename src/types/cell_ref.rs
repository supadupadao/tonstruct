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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(FromCell, ToCell, Debug, PartialEq, Default)]
    struct Message {
        test_field: bool,
    }

    #[test]
    fn test_new() {
        assert_eq!(
            CellRef::new(Message { test_field: true }),
            CellRef(Message { test_field: true })
        );
    }

    #[test]
    fn test_into_inner() {
        assert_eq!(
            CellRef(Message { test_field: true }).into_inner(),
            Message { test_field: true }
        );
    }

    #[test]
    fn test_inner() {
        assert_eq!(
            CellRef(Message { test_field: true }).inner(),
            &Message { test_field: true }
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            CellRef::<Message>::default(),
            CellRef(Message { test_field: false })
        );
    }

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new()
            .store_reference(
                &CellBuilder::new()
                    .store_bit(true)
                    .unwrap()
                    .build()
                    .unwrap()
                    .to_arc(),
            )
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            CellRef::<Message>::from_cell(cell).unwrap(),
            CellRef(Message { test_field: true })
        );
    }

    #[test]
    fn test_to_cell() {
        assert_eq!(
            CellRef(Message { test_field: true }).to_cell().unwrap(),
            CellBuilder::new()
                .store_reference(
                    &CellBuilder::new()
                        .store_bit(true)
                        .unwrap()
                        .build()
                        .unwrap()
                        .to_arc(),
                )
                .unwrap()
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell() {
        let first_iter = CellRef::new(Message { test_field: true });
        let cell = first_iter.to_cell().unwrap();
        let second_iter = CellRef::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
