use num_bigint::BigUint;
use std::ops::Deref;
use std::str::FromStr;
use tonlib_core::cell::BagOfCells;
use tonlib_core::TonAddress;
use tonstruct::fields::{Address, CellRef, Coins, Comment, Int, Uint};
use tonstruct::FromCell;

const JETTON_TRANSFER_OP_CODE: u32 = 0x0f8a7ea5;

#[test]
fn test_de_jetton_transfer_message() {
    // Transaction 15554a34e28fe3bb58e2e0deab4a49c4da3d149ce88665cb253701492fbb97b2
    #[derive(FromCell, Debug, PartialEq)]
    struct ForwardPayload {
        is_right: bool,
        text_comment: CellRef<Comment>,
    }

    #[derive(FromCell, Debug, PartialEq)]
    struct JettonTransfer {
        op_code: Uint<32>,
        query_id: Uint<64>,
        amount: Coins,
        destination: Address,
        response_destination: Address,
        custom_payload: Option<Int<0>>,
        forward_ton_amount: Coins,
        forward_payload: ForwardPayload,
    }

    const JETTON_BODY: &str = "b5ee9c720101020100760001aa0f8a7ea50000000067b0b2db407f819a080146df30c28c100449854efcf8863cb79a60c5b74239a6c01e2e8157be74d069e70028dbe6185182008930a9df9f10c796f34c18b6e84734d803c5d02af7ce9a0d3cc20301003800000000746573742074657374205465737420746573742074657374";

    let expected = JettonTransfer {
        op_code: Uint::from(BigUint::from(JETTON_TRANSFER_OP_CODE)),
        query_id: Uint::from(BigUint::from_str("1739633371").unwrap()),
        amount: Coins::from(BigUint::from_str("133700000").unwrap()),
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
        forward_payload: ForwardPayload {
            is_right: true,
            text_comment: CellRef::new("test test Test test test".to_string().into()),
        },
    };

    let mut boc = BagOfCells::parse_hex(JETTON_BODY).unwrap();
    let cell = boc.into_single_root().unwrap().deref().clone();

    let actual = <JettonTransfer as FromCell>::from_cell(cell).unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_de_text_comment() {
    const JETTON_BODY: &str =
        "b5ee9c7201010101001b000032000000004361707463686120636f64653a205a357751395666";

    #[derive(FromCell, Debug, PartialEq)]
    struct Message {
        text_comment: Comment,
    }

    let expected = Message {
        text_comment: "Captcha code: Z5wQ9Vf".to_string().into(),
    };

    let mut boc = BagOfCells::parse_hex(JETTON_BODY).unwrap();
    let cell = boc.into_single_root().unwrap().deref().clone();

    let actual = <Message as FromCell>::from_cell(cell).unwrap();

    assert_eq!(actual, expected)
}
