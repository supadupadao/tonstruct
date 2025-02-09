use num_bigint::{BigInt, BigUint};
use tonlib_core::cell::CellBuilder;
use tonstruct::fields::Int;
use tonstruct::ToCell;

const INT_VALUE: i32 = 0xFF;
const NEG_INT_VALUE: i32 = -0xFF;

#[test]
fn test_ser_int() {
    #[derive(ToCell)]
    struct Message {
        integer: Int<32>,
    }
    let message = Message {
        integer: Int::from(BigInt::from(INT_VALUE)),
    };
    let actual = ToCell::to_cell(&message).unwrap();

    let expected = CellBuilder::new()
        .store_bit(true)
        .unwrap()
        .store_uint(31, &BigUint::from(INT_VALUE as u32))
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_ser_neg_int() {
    #[derive(ToCell)]
    struct Message {
        integer: Int<32>,
    }
    let message = Message {
        integer: Int::from(BigInt::from(NEG_INT_VALUE)),
    };
    let actual = ToCell::to_cell(&message).unwrap();

    let expected = CellBuilder::new()
        .store_bit(false)
        .unwrap()
        .store_uint(31, &BigUint::from(INT_VALUE as u32))
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(expected, actual);
}
