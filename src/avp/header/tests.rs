use crate::avp::header::flags::*;
use crate::avp::header::Header;
use crate::common::{SliceReader, VecWriter};

#[test]
fn write_read() {
    let input = Header {
        flags: Flags::new(true, false),
        payload_length: 0,
        vendor_id: 1,
        attribute_type: 2,
    };
    let mut w = VecWriter::new();
    unsafe { input.write(&mut w) };
    let mut r = SliceReader::from(&w.data);
    let output = Header::try_read(&mut r).unwrap();
    assert_eq!(input, output);
}
