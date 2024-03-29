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
            $input.write(&mut w);

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
assigned_session_id => AVP::AssignedSessionId(0x1337.into()),
assigned_tunnel_id => AVP::AssignedTunnelId(0x1337.into()),
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
call_serial_number => AVP::CallSerialNumber(0x1337.into()),
called_number => AVP::CalledNumber("TestingNumber".to_owned().into()),
calling_number => AVP::CallingNumber("TestingNumber".to_owned().into()),
challenge => AVP::Challenge(vec![0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08].into()),
challenge_response => AVP::ChallengeResponse([0x00,0x01, 0x02, 0x03, 0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f].into()),
firmware_revision => AVP::FirmwareRevision(0x1337.into()),
framing_capabilities => AVP::FramingCapabilities(types::FramingCapabilities::new(true, true)),
framing_type => AVP::FramingType(types::FramingType::new(true, true)),
host_name => AVP::HostName("test-host.com".as_bytes().to_owned().into()),
initial_received_lcp_conf_req => AVP::InitialReceivedLcpConfReq(vec![0xde,0xad,0xbe,0xef].into()),
last_received_lcp_conf_req => AVP::LastReceivedLcpConfReq(vec![0xde,0xad,0xbe,0xef].into()),
last_sent_lcp_conf_req => AVP::LastSentLcpConfReq(vec![0xde,0xad,0xbe,0xef].into()),
maximum_bps => AVP::MaximumBps(0x13371337.into()),
message_type => AVP::MessageType(types::MessageType::IncomingCallConnected),
minimum_bps => AVP::MinimumBps(0x13371337.into()),
physical_channel_id => AVP::PhysicalChannelId([0xde,0xad,0xbe,0xef].into()),
private_group_id => AVP::PrivateGroupId(vec![0xde,0xad,0xbe,0xef].into()),
protocol_version => AVP::ProtocolVersion(types::ProtocolVersion{version: 0xf0, revision: 0x0d}),
proxy_authen_challenge => AVP::ProxyAuthenChallenge(vec![0xde,0xad,0xbe,0xef].into()),
proxy_authen_name => AVP::ProxyAuthenName(vec![0xde,0xad,0xbe,0xef].into()),
proxy_authen_response => AVP::ProxyAuthenResponse(vec![0xde,0xad,0xbe,0xef].into()),
proxy_authen_type => AVP::ProxyAuthenType(types::ProxyAuthenType::TextualUserNamePasswordExchange),
q931_cause_code => AVP::Q931CauseCode(types::Q931CauseCode{cause_code: 0xdead, cause_msg:0xff, advisory: Some("test advisory".to_owned())}),
random_vector => AVP::RandomVector([0xde,0xad,0xbe,0xef].into()),
receive_window_size => AVP::ReceiveWindowSize(0x1337.into()),
result_code => AVP::ResultCode(types::ResultCode{
    code: types::result_code::StopCcnCode::GeneralError.into(),
    error: Some(types::result_code::Error{
        error_type: types::result_code::ErrorType::Generic,
        error_message: Some(String::from("Test error"))
    })
}),
rx_connect_speed => AVP::RxConnectSpeed(0xdeadbeef.into()),
sequencing_required => AVP::SequencingRequired(types::SequencingRequired{}),
sub_address => AVP::SubAddress("subaddress-value".to_owned().into()),
tie_breaker => AVP::TieBreaker(0xdeadbeef13371337.into()),
tx_connect_speed => AVP::TxConnectSpeed(0xdeadbeef.into()),
vendor_name => AVP::VendorName("test vendor".to_owned().into())
];

#[test]
fn hidden() {
    let secret = "my_super_secret".as_bytes().to_owned();
    let length_padding = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let alignment_padding = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ];
    let rv = [0xde, 0xad, 0xbe, 0xef].into();
    let input = AVP::VendorName("test vendor".to_owned().into());

    // Serialize input
    let hidden = input
        .clone()
        .hide(&secret, &rv, &length_padding, &alignment_padding);
    let mut w = VecWriter::new();
    hidden.write(&mut w);

    println!("{:?}", w.data);

    // Deserialize to output
    let mut r = SliceReader::from(&w.data);
    let avps = AVP::try_read_greedy(&mut r);

    // Select first and only AVP, assert successful deserialization
    let output = avps.into_iter().next().unwrap().unwrap();
    assert!(matches!(output, AVP::Hidden(_)));

    // Reveal hidden
    let revealed = output.reveal(&secret, &rv).unwrap();
    assert_eq!(revealed, input);
}
