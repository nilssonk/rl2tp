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
        AVP::MessageType(types::MessageType::OutgoingCallRequest),
    ])
});
