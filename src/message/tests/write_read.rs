use crate::common::{SliceReader, VecWriter};
use crate::message::*;

#[test]
fn empty_control() {
    const TOTAL_LENGTH: usize = 12;

    let in_msg = Message::Control(ControlMessage {
        length: TOTAL_LENGTH as u16,
        tunnel_id: 0,
        session_id: 1,
        ns: 2,
        nr: 3,
        avps: Vec::new(),
    });

    let mut w = VecWriter::new();
    unsafe { in_msg.write(&mut w) };

    assert_eq!(w.data.len(), TOTAL_LENGTH);

    let r = Box::new(SliceReader::from(&w.data));
    let out_msg = Message::try_read(
        r,
        ValidationOptions {
            reserved: ValidateReserved::Yes,
            version: ValidateVersion::Yes,
            unused: ValidateUnused::Yes,
        },
    )
    .unwrap();

    assert_eq!(in_msg, out_msg);
}

#[test]
fn empty_data() {
    let total_length = 12;
    let data = Vec::new();
    let in_msg = Message::Data(DataMessage {
        is_prioritized: false,
        length: Some(total_length as u16),
        tunnel_id: 0,
        session_id: 1,
        ns_nr: Some((2, 3)),
        offset: None,
        data: &data,
    });

    let mut w = VecWriter::new();
    unsafe { in_msg.write(&mut w) };

    assert_eq!(w.data.len(), total_length);

    let r = Box::new(SliceReader::from(&w.data));
    let out_msg = Message::try_read(
        r,
        ValidationOptions {
            reserved: ValidateReserved::Yes,
            version: ValidateVersion::Yes,
            unused: ValidateUnused::Yes,
        },
    )
    .unwrap();

    assert_eq!(in_msg, out_msg);
}
