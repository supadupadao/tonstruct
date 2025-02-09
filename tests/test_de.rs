use num_bigint::BigUint;
use std::ops::Deref;
use std::str::FromStr;
use tonlib_core::cell::BagOfCells;
use tonlib_core::TonAddress;
use tonstruct::fields::{Address, Coins, Int, Uint};
use tonstruct::FromCell;

#[test]
fn test_de_jetton_transfer_message() {
    // Transaction 8917a2f87f0b9ec52673b868a7ae111c19654f5b58961b1f7034f384dc5bcb83
    const JETTON_BODY: &str = "b5ee9c720101020100cc0001b20f8a7ea5000ae06dae10e13655ab2662af4800ef3b9902a271b2a01c8938a523cfe24e71847aaeb6a620001ed44a77ac0e709d0036336667ecac6ce139b289530e20cdd788c446a2de30056e218a1b0c6fc478a7480ee6b2810100db2593856180022a16a3164c4d5aa3133f3110ff10496e00ca8ac8abeffc5027e024d33480c3ea04a221e7010036336667ecac6ce139b289530e20cdd788c446a2de30056e218a1b0c6fc478a77001b23bbd527c00fd3d0874026bb7941f8fdd4d599ed8e7a63f426d5723f0388f2e";
    const JETTON_TRANSFER_OP_CODE: u32 = 0x0f8a7ea5;

    #[derive(FromCell, Debug, PartialEq)]
    struct ForwardPayload {}

    #[derive(FromCell, Debug, PartialEq)]
    struct Message {
        op_code: Uint<32>,
        query_id: Uint<64>,
        amount: Coins,
        destination: Address,
        response_destination: Address,
        custom_payload: Option<Int<0>>,
        forward_ton_amount: Coins,
        forward_payload: ForwardPayload,
    }
    let expected = Message {
        op_code: Uint::from(BigUint::from(JETTON_TRANSFER_OP_CODE)),
        query_id: Uint::from(BigUint::from_str("3061511443505462").unwrap()),
        amount: Coins::from(BigUint::from_str("389540096756").unwrap()),
        destination: Address::from(
            TonAddress::from_base64_url("EQB3ncyBUTjZUA5EnFKR5_EnOMI9V1tTEAAPaiU71gc4TiUt")
                .unwrap(),
        ),
        response_destination: Address::from(
            TonAddress::from_base64_url("UQDYzZmfsrGzhObKJUw4gzdeIxEai3jAFbiGKGwxvxHinf4K")
                .unwrap(),
        ),
        custom_payload: None,
        forward_ton_amount: Coins::from(BigUint::from_str("125000000").unwrap()),
        forward_payload: ForwardPayload {},
    };

    let mut boc = BagOfCells::parse_hex(JETTON_BODY).unwrap();
    let cell = boc.into_single_root().unwrap().deref().clone();

    let actual = <Message as FromCell>::from_cell(cell).unwrap();

    assert_eq!(actual, expected)
}
