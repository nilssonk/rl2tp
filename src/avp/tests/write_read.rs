use crate::avp::*;
use crate::common::VecWriter;

#[test]
fn message_type() {
    let input = AVP::MessageType(types::MessageType::IncomingCallConnected);
    let mut w = VecWriter::new();
    unsafe {
        input.write(&mut w);
    }
    let r = Box::new(SliceReader::from(&w.data));
    let avps = AVP::try_read_greedy(r);
    let output = avps.into_iter().next().unwrap().unwrap();
    assert_eq!(input, output);
}
