use num_bigint::{BigInt, BigUint};
use std::ops::Deref;
use std::str::FromStr;
use tonlib_core::cell::{BagOfCells, Cell};
use tonlib_core::TonAddress;
use tonstruct::fields::{Address, Coins, Int, Uint};
use tonstruct::FromCell;

#[test]
fn test_de_jetton_transfer_message() {
    const JETTON_BODY: &str = "b5ee9c720101020100700001ae0f8a7ea51801125d220e2dd467fba097f920080146df30c28c100449854efcf8863cb79a60c5b74239a6c01e2e8157be74d069e70028dbe6185182008930a9df9f10c796f34c18b6e84734d803c5d02af7ce9a0d3cc203010028000000004a6574746f6e7320756e7374616b6564";
    const JETTON_TRANSFER_OP_CODE: u32 = 0x0f8a7ea5;

    #[derive(FromCell, Debug, PartialEq)]
    struct Message {
        op_code: Uint<32>,
        query_id: Uint<64>,
        amount: Coins,
        destination: Address,
        response_destination: Address,
        custom_payload: Option<Int<0>>,
        forward_ton_amount: Coins,
    }
    let expected = Message {
        op_code: Uint::from(BigUint::from(JETTON_TRANSFER_OP_CODE)),
        query_id: Uint::from(BigUint::from_str("1729683923099594196").unwrap()),
        amount: Coins::from(BigUint::from_str("140437000000000").unwrap()),
        destination: Address::from(
            TonAddress::from_base64_url("UQCjb5hhRggCJMKnfnxDHlvNMGLboRzTYA8XQKvfOmg08wNo")
                .unwrap(),
        ),
        response_destination: Address::from(
            TonAddress::from_base64_url("UQCjb5hhRggCJMKnfnxDHlvNMGLboRzTYA8XQKvfOmg08wNo")
                .unwrap(),
        ),
        custom_payload: None,
        forward_ton_amount: Coins::from(BigUint::from_str("1").unwrap()),
    };

    let mut boc = BagOfCells::parse_hex(JETTON_BODY).unwrap();
    let cell = boc.into_single_root().unwrap().deref().clone();

    let actual = <Message as FromCell>::from_cell(cell).unwrap();

    assert_eq!(actual, expected)
}
