#[cfg(feature = "tonlib_core")]
mod tests {
    use num_bigint::{BigInt, BigUint};
    use tonlib_core::cell::CellBuilder;
    use tonstruct::fields::{Coins, Int, Uint};
    use tonstruct::{
        fields::Address,
        tonlib_core::{from_tonlib_core_cell, to_tonlib_core_cell},
        FromCell, ToCell,
    };

    #[test]
    fn test_de_tonlib_core() {
        #[derive(FromCell, Debug, PartialEq)]
        struct Message {
            address: Address,
            boolean: bool,
            coins: Coins,
            int: Int<32>,
            uint: Uint<32>,
        }

        let expected = Message {
            address: tonlib_core::TonAddress::from_base64_url(
                "UQCjb5hhRggCJMKnfnxDHlvNMGLboRzTYA8XQKvfOmg08wNo",
            )
            .unwrap()
            .into(),
            boolean: true,
            coins: Coins::default(),
            int: Int::default(),
            uint: Uint::default(),
        };

        let cell = CellBuilder::new()
            .store_address(
                &tonlib_core::TonAddress::from_base64_url(
                    "UQCjb5hhRggCJMKnfnxDHlvNMGLboRzTYA8XQKvfOmg08wNo",
                )
                .unwrap(),
            )
            .unwrap()
            .store_bit(true)
            .unwrap()
            .store_coins(&BigUint::from(0u8))
            .unwrap()
            .store_int(32, &BigInt::from(0u8))
            .unwrap()
            .store_uint(32, &BigUint::from(0u8))
            .unwrap()
            .build()
            .unwrap();
        let actual = from_tonlib_core_cell::<Message>(cell).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_ser_tonlib_core() {
        #[derive(ToCell)]
        struct Message {
            boolean: bool,
            address: Address,
            coins: Coins,
            int: Int<32>,
            uint: Uint<32>,
        }
        let message = Message {
            boolean: true,
            address: Default::default(),
            coins: Default::default(),
            int: Default::default(),
            uint: Default::default(),
        };
        let actual = to_tonlib_core_cell(message).unwrap();

        let expected = CellBuilder::new()
            .store_bit(true)
            .unwrap()
            .store_address(&tonlib_core::TonAddress::NULL)
            .unwrap()
            .store_coins(&BigUint::from(0u8))
            .unwrap()
            .store_int(32, &BigInt::from(0u8))
            .unwrap()
            .store_uint(32, &BigUint::from(0u8))
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(expected, actual);
    }
}
