use crate::avp::*;
use crate::common::{SliceReader, VecWriter};

macro_rules! io_tests {
    [$($name:ident => $input:expr),+] => {
        $(
        #[test]
        fn $name() {
            // Serialize input
            let input = $input;
            let mut w = VecWriter::new();
            unsafe { $input.write(&mut w) };

            // Deserialize to output
            let r = Box::new(SliceReader::from(&w.data));
            let avps = AVP::try_read_greedy(r);

            // Select first and only AVP, assert successful deserialization
            let output = avps.into_iter().next().unwrap().unwrap();

            assert_eq!(input, output);
        }
    )+
    }
}

io_tests![
accm => AVP::Accm(types::Accm {
    send_accm: [0x01, 0x02, 0x03, 0x04],
    receive_accm: [0x05, 0x06, 0x07, 0x08]}),
assigned_session_id => AVP::AssignedSessionId(types::AssignedSessionId{value: 0x1337}),
assigned_tunnel_id => AVP::AssignedTunnelId(types::AssignedTunnelId{value: 0x1337}),
bearer_capabilities => AVP::BearerCapabilities(types::BearerCapabilities::new(true, true)),
bearer_type => AVP::BearerType(types::BearerType::new(true, true)),
call_errors => AVP::CallErrors(types::CallErrors{
    crc_errors: 10,
    framing_errors: 11,
    hardware_overruns: 12,
    buffer_overruns: 13,
    timeout_errors: 14,
    alignment_errors: 15
}),
call_serial_number => AVP::CallSerialNumber(types::CallSerialNumber{value: 0x1337}),
called_number => AVP::CalledNumber(types::CalledNumber{value: "TestingNumber".to_owned()}),
calling_number => AVP::CallingNumber(types::CallingNumber{value: "TestingNumber".to_owned()}),
challenge => AVP::Challenge(types::Challenge{value: vec![0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08]}),
challenge_response => AVP::ChallengeResponse(types::ChallengeResponse{data: [0x00,0x01, 0x02, 0x03, 0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f]}),
message_type => AVP::MessageType(types::MessageType::IncomingCallConnected)
];
