use disarm64_custom::decoder::decode;

#[test]
pub fn decode_successful() {
    assert!(decode(0x5400008B).is_some());
    assert!(decode(0x08DFFC20).is_some());
    assert!(decode(0x9ac32041).is_some());
}
