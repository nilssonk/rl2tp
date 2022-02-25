use crate::common::{VecWriter, Writer};

#[test]
fn write_bytes_len() {
    let input = [
        0x00, // 8
        0x01, 0x02, // 16
        0x03, 0x04, 0x05, 0x06, // 32
        0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, // 64
    ];

    let mut w = VecWriter::new();

    unsafe { w.write_bytes_unchecked(&input) };
    assert_eq!(w.len(), input.len());
}

#[test]
fn write_integers_len() {
    let mut w = VecWriter::new();

    unsafe { w.write_u8_unchecked(0x00) };
    assert_eq!(w.len(), 1);

    unsafe { w.write_u16_be_unchecked(0x0102) };
    assert_eq!(w.len(), 3);

    unsafe { w.write_u32_be_unchecked(0x03040506) };
    assert_eq!(w.len(), 7);

    unsafe { w.write_u64_be_unchecked(0x0708090a0b0c0d0e) };
    assert_eq!(w.len(), 15);
}

#[test]
fn write_bytes() {
    let input = [
        0x00, // 8
        0x01, 0x02, // 16
        0x03, 0x04, 0x05, 0x06, // 32
        0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, // 64
    ];

    let mut w = VecWriter::new();

    unsafe { w.write_bytes_unchecked(&input) };
    assert_eq!(w.data, input);
}

#[test]
fn write_integers() {
    let mut w = VecWriter::new();

    unsafe {
        w.write_u8_unchecked(0x00);
        w.write_u16_be_unchecked(0x0102);
        w.write_u32_be_unchecked(0x03040506);
        w.write_u64_be_unchecked(0x0708090a0b0c0d0e);
    }

    assert_eq!(
        w.data,
        [
            0x00, // 8
            0x01, 0x02, // 16
            0x03, 0x04, 0x05, 0x06, // 32
            0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e // 64
        ]
    );
}
