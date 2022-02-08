use crate::message::*;

#[test]
fn message_type() {
    // ControlMessage with Message Type AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
            length: 20,
            tunnel_id: 2,
            session_id: 3,
            ns: 4,
            nr: 5,
            avps: vec![AVP::MessageType(
                types::MessageType::StartControlConnectionRequest
            )],
        }))
    );
}

#[test]
fn random_vector() {
    // ControlMessage with Random Vector AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
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
        }))
    );
}

#[test]
fn result_code() {
    // ControlMessage with Result Code AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
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
        }))
    );
}

#[test]
fn protocol_version() {
    // ControlMessage with Protocol Version AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
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
        }))
    );
}

#[test]
fn framing_capabilities() {
    // ControlMessage with Framing Capabilities AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
            length: 30,
            tunnel_id: 2,
            session_id: 3,
            ns: 4,
            nr: 5,
            avps: vec![
                AVP::MessageType(types::MessageType::StartControlConnectionRequest),
                AVP::FramingCapabilities(types::FramingCapabilities::from_raw(0x000000c0))
            ],
        }))
    );

    match m {
        Ok(Message::Control(real_m)) => {
            let avp = &real_m.avps[1];
            match avp {
                AVP::FramingCapabilities(framing) => {
                    assert!(framing.is_async_framing_supported());
                    assert!(framing.is_sync_framing_supported());
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

#[test]
fn bearer_capabilities() {
    // ControlMessage with Bearer Capabilities AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
            length: 30,
            tunnel_id: 2,
            session_id: 3,
            ns: 4,
            nr: 5,
            avps: vec![
                AVP::MessageType(types::MessageType::StartControlConnectionRequest),
                AVP::BearerCapabilities(types::BearerCapabilities::from_raw(0x000000c0))
            ],
        }))
    );

    match m {
        Ok(Message::Control(real_m)) => {
            let avp = &real_m.avps[1];
            match avp {
                AVP::BearerCapabilities(bearer) => {
                    assert!(bearer.is_analog_access_supported());
                    assert!(bearer.is_digital_access_supported());
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

#[test]
fn tie_breaker() {
    // ControlMessage with Tie Breaker AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
            length: 34,
            tunnel_id: 2,
            session_id: 3,
            ns: 4,
            nr: 5,
            avps: vec![
                AVP::MessageType(types::MessageType::StartControlConnectionRequest),
                AVP::TieBreaker(0xdeadbeeff00dfade.into()),
            ],
        }))
    );
}

#[test]
fn firmware_revision() {
    // ControlMessage with Firmware Revision AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
            length: 28,
            tunnel_id: 2,
            session_id: 3,
            ns: 4,
            nr: 5,
            avps: vec![
                AVP::MessageType(types::MessageType::StartControlConnectionRequest),
                AVP::FirmwareRevision(0xf00d.into()),
            ],
        }))
    );
}

#[test]
fn host_name() {
    // ControlMessage with Host Name AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
            length: 30,
            tunnel_id: 2,
            session_id: 3,
            ns: 4,
            nr: 5,
            avps: vec![
                AVP::MessageType(types::MessageType::StartControlConnectionRequest),
                AVP::HostName(types::HostName {
                    data: vec![0xde, 0xad, 0xbe, 0xef]
                })
            ],
        }))
    );
}

#[test]
fn vendor_name() {
    // ControlMessage with Vendor Name AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
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
        }))
    );
}

#[test]
fn assigned_tunnel_id() {
    // ControlMessage with Assigned Tunnel ID AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
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
    ];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::No,
        ValidateUnused::Yes,
    );
    assert_eq!(
        m,
        Ok(Message::Control(ControlMessage {
            length: 28,
            tunnel_id: 2,
            session_id: 3,
            ns: 4,
            nr: 5,
            avps: vec![
                AVP::MessageType(types::MessageType::StartControlConnectionRequest),
                AVP::AssignedTunnelId(types::AssignedTunnelId { value: 0xdead })
            ],
        }))
    );
}
