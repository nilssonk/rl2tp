use crate::avp::{types, AVP};
use crate::common::SliceReader;
use crate::message::*;

macro_rules! read_tests {
    [$($name:ident : $input:expr => $output:expr),+] => {
        $(
        #[test]
        fn $name() {
            let data = $input;
            let result = Message::try_read(
                &mut SliceReader::from(&data),
                ValidationOptions {
                    reserved: ValidateReserved::Yes,
                    version: ValidateVersion::Yes,
                    unused: ValidateUnused::Yes
                });
            assert_eq!(result, Ok(Message::Control($output)));
        }
        )+
    }
}

macro_rules! read_tests_extended {
    [$($name:ident : $input:expr => $output:expr => $extra_test:expr),+] => {
        $(
        #[test]
        fn $name() {
            let data = $input;
            let mut result = Message::try_read(&mut SliceReader::from(&data),
                ValidationOptions {
                    reserved: ValidateReserved::Yes,
                    version: ValidateVersion::Yes,
                    unused: ValidateUnused::Yes
                });
            assert_eq!(result, Ok(Message::Control($output)));

            // Perform extended test on final decoded AVP
            let test = $extra_test;
            match result {
                Ok(Message::Control(ref mut msg)) => {
                    let avp = msg.avps.pop().unwrap();
                    test(avp);
                },
                _ => panic!(),
            }
        }
        )+
    }
}

read_tests![
    message_type:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x14, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
    ] => ControlMessage {
        length: 20,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![AVP::MessageType(
            types::MessageType::StartControlConnectionRequest
        )],
    },
    random_vector:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x24, // Attribute Type (Random Vector)
        0xde, 0xad, // Random Vector payload
        0xbe, 0xef,
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::RandomVector(types::RandomVector {
                data: [0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    result_code:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x28, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x04, // Type 4 (StopControlConnectionNotification)
        // AVP Payload
        0x00, 0x14, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x01, // Attribute Type (ResultCode)
        0x00, 0x02, // Result Code (StopCCN General Error)
        0x00, 0x06, // Error Code (Generic error)
        0x54, 0x65, 0x73, 0x74, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, // "Test error"
    ] => ControlMessage {
        length: 40,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StopControlConnectionNotification),
            AVP::ResultCode(types::ResultCode {
                code: types::result_code::StopCcnCode::GeneralError.into(),
                error: Some(types::result_code::Error::Generic),
                error_message: Some(String::from("Test error"))
            })
        ],
    },
    protocol_version:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1c, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x02, // Attribute Type (Protocol Version)
        0xbe, // Version
        0xef, // Revision
    ] => ControlMessage {
        length: 28,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::ProtocolVersion(types::ProtocolVersion {
                version: 0xbe,
                revision: 0xef
            })
        ],
    },
    tie_breaker:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x22, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x0e, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x05, // Attribute Type (Tie Breaker)
        0xde, 0xad, 0xbe, 0xef, 0xf0, 0x0d, 0xfa, 0xde, // Tie breaker value
    ] => ControlMessage {
        length: 34,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::TieBreaker(0xdeadbeeff00dfade.into()),
        ],
    },
    firmware_revision:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1c, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x06, // Attribute Type (Firmware Revision)
        0xf0, 0x0d, // Firmware Revision
    ] => ControlMessage {
        length: 28,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::FirmwareRevision(0xf00d.into()),
        ],
    },
    host_name:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x07, // Attribute Type (Host Name)
        0xde, 0xad, // Host Name
        0xbe, 0xef,
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::HostName(types::HostName {
                value: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    vendor_name:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x08, // Attribute Type (Vendor Name)
        0xde, 0xad, // Vendor Name
        0xbe, 0xef,
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::VendorName(types::VendorName {
                data: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    assigned_tunnel_id:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1c, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x09, // Attribute Type (Assigned Tunnel ID)
        0xde, 0xad, // Assigned Tunnel ID
    ] => ControlMessage {
        length: 28,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::AssignedTunnelId(types::AssignedTunnelId { value: 0xdead })
        ],
    },
    receive_window_size:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1c, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x0a, // Attribute Type (Receive Window Size)
        0xde, 0xad, // Receive Window Size
    ] =>
    ControlMessage {
        length: 28,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::ReceiveWindowSize(types::ReceiveWindowSize { value: 0xdead })
        ],
    },
    challenge:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x0b, // Attribute Type (Challenge)
        0xde, 0xad, 0xbe, 0xef, // Challenge
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::Challenge(types::Challenge {
                value: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    challenge_response:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x2a, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x16, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x0d, // Attribute Type (Challenge Response)
        0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe,
        0xef, // Challenge Response
    ] =>
    ControlMessage {
        length: 42,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::ChallengeResponse(types::ChallengeResponse {
                data: [
                    0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef,
                    0xde, 0xad, 0xbe, 0xef
                ]
            })
        ],
    },
    q931_cause_code:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x27, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0e, // Type 14 (CallDisconnectNotify)
        // AVP Payload
        0x00, 0x13, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x0c, // Attribute Type (Q.931 Cause Code)
        // Q.931 Cause Code
        0x01, 0x02, // Cause Code
        0x03, // Cause Msg
        0x54, 0x65, 0x73, 0x74, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, // "Test error"
    ] => ControlMessage {
        length: 39,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::CallDisconnectNotify),
            AVP::Q931CauseCode(types::Q931CauseCode {
                cause_code: 0x0102,
                cause_msg: 0x03,
                advisory: Some(String::from("Test error")),
            })
        ],
    },
    assigned_session_id:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1c, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0e, // Type 14 (CallDisconnectNotify)
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x0e, // Attribute Type (Assigned Session ID)
        0xde, 0xad, // Assigned Session ID
    ] => ControlMessage {
        length: 28,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::CallDisconnectNotify),
            AVP::AssignedSessionId(types::AssignedSessionId { value: 0xdead })
        ],
    },
    call_serial_number:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x07, // Type 7 (OutgoingCallRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x0f, // Attribute Type (Call Serial Number)
        0xde, 0xad, 0xbe, 0xef, // Call Serial Number
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::OutgoingCallRequest),
            AVP::CallSerialNumber(types::CallSerialNumber { value: 0xdeadbeef })
        ],
    },
    minimum_bps:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x07, // Type 7 (OutgoingCallRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x10, // Attribute Type (Minimum BPS)
        0xde, 0xad, 0xbe, 0xef, // Minimum BPS
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::OutgoingCallRequest),
            AVP::MinimumBps(types::MinimumBps { value: 0xdeadbeef })
        ],
    },
    maximum_bps:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x07, // Type 7 (OutgoingCallRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x11, // Attribute Type (MaximumBps)
        0xde, 0xad, 0xbe, 0xef, // MaximumBps
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::OutgoingCallRequest),
            AVP::MaximumBps(types::MaximumBps { value: 0xdeadbeef })
        ],
    },
    called_number:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x25, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x07, // Type 7 (OutgoingCallRequest)
        // AVP Payload
        0x00, 0x11, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x15, // Attribute Type (Called Number)
        0x54, 0x65, 0x73, 0x74, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, // Called Number
    ] => ControlMessage {
        length: 37,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::OutgoingCallRequest),
            AVP::CalledNumber(types::CalledNumber {
                value: "Test number".to_owned()
            })
        ],
    },
    calling_number:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x25, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0a, // Type 10 (IncomingCallRequest)
        // AVP Payload
        0x00, 0x11, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x16, // Attribute Type (Calling Number)
        0x54, 0x65, 0x73, 0x74, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, // Calling Number
    ] => ControlMessage {
        length: 37,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallRequest),
            AVP::CallingNumber(types::CallingNumber {
                value: "Test number".to_owned()
            })
        ],
    },
    sub_address:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x26, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0a, // Type 10 (IncomingCallRequest)
        // AVP Payload
        0x00, 0x12, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x17, // Attribute Type (Sub-Address)
        0x54, 0x65, 0x73, 0x74, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, // Sub-Address
    ] => ControlMessage {
        length: 38,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallRequest),
            AVP::SubAddress(types::SubAddress {
                value: "Test address".to_owned()
            })
        ],
    },
    tx_connect_speed:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x18, // Attribute Type (Tx Connect Speed)
        0xde, 0xad, 0xbe, 0xef, // Tx Connect Speed
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::TxConnectSpeed(types::TxConnectSpeed { value: 0xdeadbeef })
        ],
    },
    rx_connect_speed:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x26, // Attribute Type (Rx Connect Speed)
        0xde, 0xad, 0xbe, 0xef, // Rx Connect Speed
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::RxConnectSpeed(types::RxConnectSpeed { value: 0xdeadbeef })
        ],
    },
    physical_channel_id:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0a, // Type 10 (IncomingCallRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x19, // Attribute Type (Physical Channel ID)
        0xde, 0xad, 0xbe, 0xef, // Physical Channel ID
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallRequest),
            AVP::PhysicalChannelId(types::PhysicalChannelId {
                value: [0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    private_group_id:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x25, // Attribute Type (Private Group ID)
        0xde, 0xad, 0xbe, 0xef, // Private Group ID
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::PrivateGroupId(types::PrivateGroupId {
                data: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    sequencing_required:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1a, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x06, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x27, // Attribute Type (Sequencing Required)
    ] => ControlMessage {
        length: 26,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::SequencingRequired(types::SequencingRequired::default()),
        ],
    },
    initial_received_lcp_conf_req:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x1a, // Attribute Type (Initial Received LCP CONFREQ)
        0xde, 0xad, 0xbe, 0xef, // Initial Received LCP CONFREQ
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq {
                value: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    last_sent_lcp_conf_req:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x1b, // Attribute Type (Last Sent LCP CONFREQ)
        0xde, 0xad, 0xbe, 0xef, // Last Sent LCP CONFREQ
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::LastSentLcpConfReq(types::LastSentLcpConfReq {
                value: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    last_received_lcp_conf_req:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x1c, // Attribute Type (Last Received LCP CONFREQ)
        0xde, 0xad, 0xbe, 0xef, // Last Received LCP CONFREQ
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::LastReceivedLcpConfReq(types::LastReceivedLcpConfReq {
                value: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    proxy_authen_type:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1c, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x1d, // Attribute Type (Proxy Authen Type)
        0x00, 0x05, // MicrosoftChapVersion1
    ] => ControlMessage {
        length: 28,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::ProxyAuthenType(types::ProxyAuthenType::MicrosoftChapVersion1)
        ],
    },
    proxy_authen_name:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x1e, // Attribute Type (Proxy Authen Name)
        0xde, 0xad, 0xbe, 0xef, // Proxy Authen Name
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::ProxyAuthenName(types::ProxyAuthenName {
                data: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    proxy_authen_challenge:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x1f, // Attribute Type (Proxy Authen Challenge)
        0xde, 0xad, 0xbe, 0xef, // Proxy Authen Challenge
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::ProxyAuthenChallenge(types::ProxyAuthenChallenge {
                data: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    proxy_authen_id:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1c, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x20, // Attribute Type (Proxy Authen ID)
        0x00, 0xff, // Proxy Authen ID
    ] => ControlMessage {
        length: 28,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::ProxyAuthenId(0xff.into())
        ],
    },
    proxy_authen_response:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0c, // Type 12 (IncomingCallConnected)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x21, // Attribute Type (Proxy Authen Response)
        0xde, 0xad, 0xbe, 0xef, // Proxy Authen Response
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::IncomingCallConnected),
            AVP::ProxyAuthenResponse(types::ProxyAuthenResponse {
                data: vec![0xde, 0xad, 0xbe, 0xef]
            })
        ],
    },
    call_errors:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x34, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x0f, // Type 15 (WanErrorNotify)
        // AVP Payload
        0x00, 0x20, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x22, // Attribute Type (Call Errors)
        // Call Errors
        0x00, 0x00, // Reserved
        0xde, 0xad, 0xbe, 0xef, // CRC Errors
        0xf0, 0x0d, 0xfa, 0xde, // Framing Errors
        0xda, 0xda, 0xb0, 0xb0, // Hardware Overruns
        0xff, 0xff, 0xcc, 0xcc, // Buffer Overruns
        0xaa, 0xbb, 0xcc, 0xdd, // Time-out Errors
        0x11, 0x22, 0x33, 0x44, // Alignment Errors
    ] => ControlMessage {
        length: 52,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::WanErrorNotify),
            AVP::CallErrors(types::CallErrors {
                crc_errors: 0xdeadbeef,
                framing_errors: 0xf00dfade,
                hardware_overruns: 0xdadab0b0,
                buffer_overruns: 0xffffcccc,
                timeout_errors: 0xaabbccdd,
                alignment_errors: 0x11223344
            })
        ],
    },
    accm:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x24, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x10, // Type 15 (SetLinkInfo)
        // AVP Payload
        0x00, 0x10, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x23, // Attribute Type (ACCM)
        // ACCM
        0x00, 0x00, // Reserved
        0xde, 0xad, 0xbe, 0xef, // Send ACCM
        0xf0, 0x0d, 0xfa, 0xde, // Receive ACCM
    ] => ControlMessage {
        length: 36,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::SetLinkInfo),
            AVP::Accm(types::Accm {
                send_accm: [0xde, 0xad, 0xbe, 0xef],
                receive_accm: [0xf0, 0x0d, 0xfa, 0xde],
            })
        ],
    }
];

read_tests_extended![
    framing_capabilities:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x03, // Attribute Type (Framing Capabilities)
        0x00, 0x00, 0x00, 0xc0, // Async and sync framing supported
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::FramingCapabilities(types::FramingCapabilities::new(true,true))
        ],
    } => |avp: AVP| {
        match avp {
            AVP::FramingCapabilities(framing) => {
            assert!(framing.is_async_framing_supported());
            assert!(framing.is_sync_framing_supported());
            },
            _ => panic!()
        }
    },
    bearer_capabilities:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x04, // Attribute Type (Bearer Capabilities)
        0x00, 0x00, 0x00, 0xc0, // Digital and analog access supported
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::StartControlConnectionRequest),
            AVP::BearerCapabilities(types::BearerCapabilities::from_raw(0x000000c0))
        ],
    } => |avp: AVP| {
        match avp {
            AVP::BearerCapabilities(bearer) => {
                assert!(bearer.is_analog_access_supported());
                assert!(bearer.is_digital_access_supported());
            },
            _ => panic!(),
        }
    },
    bearer_type:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x07, // Type 7 (OutgoingCallRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x12, // Attribute Type (Bearer Type)
        0x00, 0x00, 0x00, 0xc0, // Digital and analog access requested
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::OutgoingCallRequest),
            AVP::BearerType(types::BearerType::from_raw(0x000000c0))
        ],
    } => |avp: AVP| {
        match avp {
            AVP::BearerType(bearer) => {
                assert!(bearer.is_analog_request());
                assert!(bearer.is_digital_request());
            }
            _ => panic!(),
        }
    },
    framing_type:
    vec![
        0x13, 0x20, // Flags
        0x00, 0x1e, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // AVP Payload
        0x00, 0x08, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x00, // Attribute Type (Message Type)
        0x00, 0x07, // Type 7 (OutgoingCallRequest)
        // AVP Payload
        0x00, 0x0a, // Flags and Length
        0x00, 0x00, // Vendor ID
        0x00, 0x13, // Attribute Type (Framing Type)
        0x00, 0x00, 0x00, 0xc0, // Digital and analog framing requested
    ] => ControlMessage {
        length: 30,
        tunnel_id: 2,
        session_id: 3,
        ns: 4,
        nr: 5,
        avps: vec![
            AVP::MessageType(types::MessageType::OutgoingCallRequest),
            AVP::FramingType(types::FramingType::new(true, true))
        ],
    } =>
    |avp: AVP|
    match avp {
        AVP::FramingType(bearer) => {
            assert!(bearer.is_analog_request());
            assert!(bearer.is_digital_request());
        }
        _ => panic!(),
    }
];
