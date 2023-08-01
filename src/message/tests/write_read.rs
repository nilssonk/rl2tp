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
    in_msg.write(&mut w);

    assert_eq!(w.data.len(), TOTAL_LENGTH);

    let out_msg = Message::try_read_validate(
        &mut SliceReader::from(&w.data),
        ValidationOptions {
            reserved: ValidateReserved::Yes,
            version: ValidateVersion::Yes,
            unused: ValidateUnused::Yes,
        },
    )
    .unwrap();

    assert_eq!(in_msg, out_msg);
}
