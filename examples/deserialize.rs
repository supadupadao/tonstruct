//! In this example we will parse
//! the `15554a34e28fe3bb58e2e0deab4a49c4da3d149ce88665cb253701492fbb97b2` transaction
//!
//! https://tonviewer.com/transaction/15554a34e28fe3bb58e2e0deab4a49c4da3d149ce88665cb253701492fbb97b2
//!
//! This transaction is regular TON jetton transfer, specifically `JettonTransfer` message (0x0f8a7ea5)

use std::ops::Deref;
use tonlib_core::cell::BagOfCells;
use tonstruct::fields::{Address, CellRef, Coins, Comment, Int, Uint};
use tonstruct::FromCell;

/// Transaction body. Click "Copy Raw body" on TonViewer to get it
static BASE64_BODY: &str = "b5ee9c720101020100760001aa0f8a7ea50000000067b0b2db407f819a080146df30c28c100449854efcf8863cb79a60c5b74239a6c01e2e8157be74d069e70028dbe6185182008930a9df9f10c796f34c18b6e84734d803c5d02af7ce9a0d3cc20301003800000000746573742074657374205465737420746573742074657374";

#[derive(FromCell)]
struct ForwardPayload {
    _is_right: bool,
    _text_comment: CellRef<Comment>,
}

#[derive(FromCell)]
struct JettonTransfer {
    _op_code: Uint<32>,
    _query_id: Uint<64>,
    _amount: Coins,
    _destination: Address,
    _response_destination: Address,
    _custom_payload: Option<Int<0>>,
    _forward_ton_amount: Coins,
    _forward_payload: ForwardPayload,
}

fn main() {
    // First we need to parse base64 body to `tonlib_code::Cell` struct
    let mut boc = BagOfCells::parse_hex(BASE64_BODY).unwrap();
    let cell = boc.into_single_root().unwrap();

    // Then just pass it to struct deserializer
    let _ = JettonTransfer::from_cell(cell.deref().clone());
}
