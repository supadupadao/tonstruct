use serde::Serialize;
use serde_ton::types::address::Address;
use serde_ton::types::int::Int;
use serde_ton::CellSerializer;
use tonlib_core::cell::CellBuilder;
use tonlib_core::TonAddress;

pub const BOOL_VALUE: bool = true;
pub const UINT8_VALUE: u8 = 0x12;
pub const UINT16_VALUE: u16 = 0x1234;
pub const UINT32_VALUE: u32 = 0x12345678;
pub const UINT64_VALUE: u64 = 0x1234567890ABCDEF;
pub const INT8_VALUE: i8 = 0x12;
pub const INT16_VALUE: i16 = 0x1234;
pub const INT32_VALUE: i32 = 0x12345678;
pub const INT64_VALUE: i64 = 0x1234567890ABCDEF;
pub static STR_VALUE: &str = "Hello, World!";

#[test]
fn test_ser_struct() {
    #[derive(Serialize)]
    struct Message {
        bool: bool,
        int8: i8,
        int16: i16,
        int32: i32,
        int64: i64,
        uint8: u8,
        uint16: u16,
        uint32: u32,
        uint64: u64,
        string: String,
    }
    let message = Message {
        bool: BOOL_VALUE,
        int8: INT8_VALUE,
        int16: INT16_VALUE,
        int32: INT32_VALUE,
        int64: INT64_VALUE,
        uint8: UINT8_VALUE,
        uint16: UINT16_VALUE,
        uint32: UINT32_VALUE,
        uint64: UINT64_VALUE,
        string: STR_VALUE.to_string(),
    };
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_bit(BOOL_VALUE)
        .unwrap()
        .store_i8(8, INT8_VALUE)
        .unwrap()
        .store_i32(16, INT16_VALUE as i32)
        .unwrap()
        .store_i32(32, INT32_VALUE)
        .unwrap()
        .store_i64(64, INT64_VALUE)
        .unwrap()
        .store_u8(8, UINT8_VALUE)
        .unwrap()
        .store_u32(16, UINT16_VALUE as u32)
        .unwrap()
        .store_u32(32, UINT32_VALUE)
        .unwrap()
        .store_u64(64, UINT64_VALUE)
        .unwrap()
        .store_reference(
            &CellBuilder::new()
                .store_u8(8, 0)
                .unwrap()
                .store_string(STR_VALUE)
                .unwrap()
                .build()
                .unwrap()
                .to_arc(),
        )
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_ser_bool() {
    #[derive(Serialize)]
    struct Message(bool);
    let message = Message(BOOL_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_bit(BOOL_VALUE)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_int8() {
    #[derive(Serialize)]
    struct Message(i8);
    let message = Message(INT8_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_i8(8, INT8_VALUE)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_int16() {
    #[derive(Serialize)]
    struct Message(i16);
    let message = Message(INT16_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_i32(16, INT16_VALUE as i32)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_int32() {
    #[derive(Serialize)]
    struct Message(i32);
    let message = Message(INT32_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_i32(32, INT32_VALUE)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_int64() {
    #[derive(Serialize)]
    struct Message(i64);
    let message = Message(INT64_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_i64(64, INT64_VALUE)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_uint8() {
    #[derive(Serialize)]
    struct Message(u8);
    let message = Message(UINT8_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_u8(8, UINT8_VALUE)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_uint16() {
    #[derive(Serialize)]
    struct Message(u16);
    let message = Message(UINT16_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_u32(16, UINT16_VALUE as u32)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_uint32() {
    #[derive(Serialize)]
    struct Message(u32);
    let message = Message(UINT32_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_u32(32, UINT32_VALUE)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_uint64() {
    #[derive(Serialize)]
    struct Message(u64);
    let message = Message(UINT64_VALUE);
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_u64(64, UINT64_VALUE)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn test_ser_str() {
    #[derive(Serialize)]
    struct Message(String);
    let message = Message(STR_VALUE.to_owned());
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_reference(
            &CellBuilder::new()
                .store_u8(8, 0)
                .unwrap()
                .store_string(STR_VALUE)
                .unwrap()
                .build()
                .unwrap()
                .to_arc(),
        )
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn ser_int_bit() {
    #[derive(Serialize)]
    struct Message(Int<4>);
    let message = Message(Int::from_usize(0xFF));
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_u8(4, 0x0F)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}

#[test]
fn ser_int_address() {
    let addr = TonAddress::null();

    #[derive(Serialize)]
    struct Message(Address);
    let message = Message(Address::new(addr.clone()));
    let actual = CellSerializer::to_cell(message).unwrap();

    let expected = CellBuilder::new()
        .store_raw_address(&addr)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(actual, expected)
}
