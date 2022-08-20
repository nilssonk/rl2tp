use rl2tp::avp::types;
use rl2tp::avp::AVP;

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static AVP_CORPUS: Lazy<Mutex<Vec<AVP>>> = Lazy::new(|| {
    Mutex::new(vec![
        AVP::AssignedTunnelId(types::AssignedTunnelId { value: 0x1337 }),
        AVP::BearerCapabilities(types::BearerCapabilities::new(true, true)),
        AVP::BearerType(types::BearerType::new(true, true)),
        AVP::CallErrors(types::CallErrors {
            crc_errors: 10,
            framing_errors: 11,
            hardware_overruns: 12,
            buffer_overruns: 13,
            timeout_errors: 14,
            alignment_errors: 15,
        }),
        AVP::CallSerialNumber(types::CallSerialNumber { value: 0x1337 }),
        AVP::CalledNumber(types::CalledNumber {
            value: "TestingNumber".to_owned(),
        }),
        AVP::CallingNumber(types::CallingNumber {
            value: "TestingNumber".to_owned(),
        }),
        AVP::ChallengeResponse(types::ChallengeResponse {
            value: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f,
            ],
        }),
        AVP::Challenge(types::Challenge {
            value: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        }),
        AVP::FirmwareRevision(types::FirmwareRevision { value: 0x1337 }),
        AVP::FramingCapabilities(types::FramingCapabilities::new(true, true)),
        AVP::FramingType(types::FramingType::new(true, true)),
        AVP::HostName(types::HostName {
            value: "test-host.com".as_bytes().to_owned(),
        }),
        AVP::InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq {
            value: vec![0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::LastReceivedLcpConfReq(types::LastReceivedLcpConfReq {
            value: vec![0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::LastSentLcpConfReq(types::LastSentLcpConfReq {
            value: vec![0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::MaximumBps(types::MaximumBps { value: 0x13371337 }),
        AVP::MessageType(types::MessageType::OutgoingCallRequest),
        AVP::MinimumBps(types::MinimumBps { value: 0x13371337 }),
        AVP::PhysicalChannelId(types::PhysicalChannelId {
            value: [0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::PrivateGroupId(types::PrivateGroupId {
            value: vec![0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::ProtocolVersion(types::ProtocolVersion {
            version: 0xf0,
            revision: 0x0d,
        }),
        AVP::ProxyAuthenChallenge(types::ProxyAuthenChallenge {
            value: vec![0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::ProxyAuthenName(types::ProxyAuthenName {
            value: vec![0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::ProxyAuthenResponse(types::ProxyAuthenResponse {
            value: vec![0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::ProxyAuthenType(types::ProxyAuthenType::TextualUserNamePasswordExchange),
        AVP::Q931CauseCode(types::Q931CauseCode {
            cause_code: 0xdead,
            cause_msg: 0xff,
            advisory: Some("test advisory".to_owned()),
        }),
        AVP::RandomVector(types::RandomVector {
            value: [0xde, 0xad, 0xbe, 0xef],
        }),
        AVP::ReceiveWindowSize(types::ReceiveWindowSize { value: 0x1337 }),
        AVP::ResultCode(types::ResultCode {
            code: types::result_code::StopCcnCode::GeneralError.into(),
            error: Some(types::result_code::Error {
                error_type: types::result_code::ErrorType::Generic,
                error_message: Some(String::from("Test error")),
            }),
        }),
        AVP::RxConnectSpeed(types::RxConnectSpeed { value: 0xdeadbeef }),
        AVP::SequencingRequired(types::SequencingRequired {}),
        AVP::SubAddress(types::SubAddress {
            value: "subaddress-value".to_owned(),
        }),
        AVP::TieBreaker(types::TieBreaker {
            value: 0xdeadbeef13371337,
        }),
        AVP::TxConnectSpeed(types::TxConnectSpeed { value: 0xdeadbeef }),
        AVP::VendorName(types::VendorName {
            value: "test vendor".to_owned(),
        }),
    ])
});
