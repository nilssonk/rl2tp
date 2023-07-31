use super::SliceReader;
use crate::common::Reader;

#[test]
fn read_integers() {
    let input = [
        0x00, // 8
        0x01, 0x02, // 16
        0x03, 0x04, 0x05, 0x06, // 32
    ];
    let mut r = SliceReader::from(&input);

    assert_eq!(r.len(), input.len());
    unsafe {
        assert_eq!(r.read_u8_unchecked(), 0x00);
        assert_eq!(r.read_u16_be_unchecked(), 0x0102);
        assert_eq!(r.read_u32_be_unchecked(), 0x03040506);
    }
}

#[test]
fn subreader() {
    let input = [
        0x00, // 8
        0x01, 0x02, // 16
        0x03, 0x04, 0x05, 0x06, // 32
    ];
    let mut r = SliceReader::from(&input);
    assert!(!r.is_empty());
    assert_eq!(r.len(), input.len());

    let mut r8 = r.subreader(1);
    let mut r16 = r.subreader(2);
    let mut r32 = r.subreader(4);

    assert!(r.is_empty());
    assert_eq!(r.len(), 0);

    assert!(!r8.is_empty());
    assert_eq!(r8.len(), 1);

    assert!(!r16.is_empty());
    assert_eq!(r16.len(), 2);

    assert!(!r32.is_empty());
    assert_eq!(r32.len(), 4);

    unsafe {
        assert_eq!(r8.read_u8_unchecked(), 0x00);
        assert_eq!(r16.read_u16_be_unchecked(), 0x0102);
        assert_eq!(r32.read_u32_be_unchecked(), 0x03040506);
    }

    assert!(r8.is_empty());
    assert_eq!(r8.len(), 0);

    assert!(r16.is_empty());
    assert_eq!(r16.len(), 0);

    assert!(r32.is_empty());
    assert_eq!(r32.len(), 0);
}
