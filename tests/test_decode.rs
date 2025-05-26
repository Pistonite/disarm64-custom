use disarm64_custom::decoder::decode;

#[test]
pub fn decode_successful() {
    assert!(decode(0x5400008B).is_some());
}
