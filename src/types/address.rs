use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use tonlib_core::cell::{CellBuilder, CellParser};
use tonlib_core::TonAddress;

#[derive(Debug, PartialEq)]
pub struct Address(TonAddress);

impl From<TonAddress> for Address {
    fn from(value: TonAddress) -> Self {
        Self(value)
    }
}

impl Into<TonAddress> for Address {
    fn into(self) -> TonAddress {
        self.0
    }
}

impl Default for Address {
    fn default() -> Self {
        Self(TonAddress::NULL)
    }
}

impl FromCell for Address {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_address().map(Self).map_err_to_anyhow()
    }
}

impl ToCell for Address {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_address(&self.0).map_err_to_anyhow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Address::from(TonAddress::NULL), Address(TonAddress::NULL));
    }

    #[test]
    fn test_into() {
        assert_eq!(
            <Address as Into<TonAddress>>::into(Address(TonAddress::NULL)),
            TonAddress::NULL
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Address::default(), Address(TonAddress::NULL));
    }

    #[test]
    fn test_from_cell() {
        let sample_address =
            TonAddress::from_base64_url("UQDYzZmfsrGzhObKJUw4gzdeIxEai3jAFbiGKGwxvxHinf4K")
                .unwrap();

        let cell = CellBuilder::new()
            .store_address(&sample_address)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(Address::from_cell(cell).unwrap(), Address(sample_address));
    }

    #[test]
    fn test_to_cell() {
        let sample_address =
            TonAddress::from_base64_url("UQDYzZmfsrGzhObKJUw4gzdeIxEai3jAFbiGKGwxvxHinf4K")
                .unwrap();

        let address = Address(sample_address.clone());
        assert_eq!(
            address.to_cell().unwrap(),
            CellBuilder::new()
                .store_address(&sample_address)
                .unwrap()
                .build()
                .unwrap()
        );
    }

    #[test]
    fn test_from_to_cell() {
        let sample_address =
            TonAddress::from_base64_url("UQDYzZmfsrGzhObKJUw4gzdeIxEai3jAFbiGKGwxvxHinf4K")
                .unwrap();

        let first_iter = Address::from(sample_address.clone());
        let cell = first_iter.to_cell().unwrap();
        let second_iter = Address::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
