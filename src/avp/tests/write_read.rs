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
            let mut r = SliceReader::from(&w.data);
            let avps = AVP::try_read_greedy(&mut r);

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
firmware_revision => AVP::FirmwareRevision(types::FirmwareRevision{value: 0x1337}),
framing_capabilities => AVP::FramingCapabilities(types::FramingCapabilities::new(true, true)),
framing_type => AVP::FramingType(types::FramingType::new(true, true)),
host_name => AVP::HostName(types::HostName{value: "test-host.com".as_bytes().to_owned() }),
initial_received_lcp_conf_req => AVP::InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq{value: vec![0xde,0xad,0xbe,0xef]}),
last_received_lcp_conf_req => AVP::LastReceivedLcpConfReq(types::LastReceivedLcpConfReq{value:vec![0xde,0xad,0xbe,0xef]}),
last_sent_lcp_conf_req => AVP::LastSentLcpConfReq(types::LastSentLcpConfReq{value:vec![0xde,0xad,0xbe,0xef]}),
maximum_bps => AVP::MaximumBps(types::MaximumBps{value:0x13371337}),
message_type => AVP::MessageType(types::MessageType::IncomingCallConnected),
minimum_bps => AVP::MinimumBps(types::MinimumBps{value:0x13371337}),
physical_channel_id => AVP::PhysicalChannelId(types::PhysicalChannelId{value:[0xde,0xad,0xbe,0xef]}),
private_group_id => AVP::PrivateGroupId(types::PrivateGroupId{value:vec![0xde,0xad,0xbe,0xef]}),
protocol_version => AVP::ProtocolVersion(types::ProtocolVersion{version: 0xf0, revision: 0x0d}),
proxy_authen_challenge => AVP::ProxyAuthenChallenge(types::ProxyAuthenChallenge{value:vec![0xde,0xad,0xbe,0xef]}),
proxy_authen_name => AVP::ProxyAuthenName(types::ProxyAuthenName{value:vec![0xde,0xad,0xbe,0xef]}),
proxy_authen_response => AVP::ProxyAuthenResponse(types::ProxyAuthenResponse{value:vec![0xde,0xad,0xbe,0xef]}),
proxy_authen_type => AVP::ProxyAuthenType(types::ProxyAuthenType::TextualUserNamePasswordExchange),
q931_cause_code => AVP::Q931CauseCode(types::Q931CauseCode{cause_code: 0xdead, cause_msg:0xff, advisory: Some("test advisory".to_owned())}),
random_vector => AVP::RandomVector(types::RandomVector{value: [0xde,0xad,0xbe,0xef]}),
receive_window_size => AVP::ReceiveWindowSize(types::ReceiveWindowSize{value: 0x1337}),
result_code => AVP::ResultCode(types::ResultCode{
    code: types::result_code::StopCcnCode::GeneralError.into(),
    error: Some(types::result_code::Error{
        error_type: types::result_code::ErrorType::Generic,
        error_message: Some(String::from("Test error"))
    })
}),
rx_connect_speed => AVP::RxConnectSpeed(types::RxConnectSpeed{value: 0xdeadbeef}),
sequencing_required => AVP::SequencingRequired(types::SequencingRequired{}),
sub_address => AVP::SubAddress(types::SubAddress{value: "subaddress-value".to_owned()}),
tie_breaker => AVP::TieBreaker(types::TieBreaker{value: 0xdeadbeef13371337}),
tx_connect_speed => AVP::TxConnectSpeed(types::TxConnectSpeed{value: 0xdeadbeef}),
vendor_name => AVP::VendorName(types::VendorName{value: "test vendor".to_owned()})
];
