use crate::message::*;

#[test]
fn from_bytes_data_valid() {
    // Data message
    let input = vec![0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0xff];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::Yes,
        ValidateUnused::No,
    );
    assert_eq!(
        m,
        Ok(Message::Data(DataMessage {
            length: None,
            tunnel_id: 0,
            session_id: 0,
            ns_nr: None,
            offset: None,
            data: &input[6..]
        }))
    );
}

#[test]
fn from_bytes_data_invalid_version() {
    // Data message with invalid version
    let input = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff];
    let m = Message::from_bytes(
        &input,
        ValidateReserved::No,
        ValidateVersion::Yes,
        ValidateUnused::No,
    );
    assert_eq!(m, Err("Invalid version encountered"));
}

#[test]
fn from_bytes_control_valid() {
    // ControlMessage with Message Type AVP
    use crate::avp::{types, AVP};
    let input = vec![
        0x13, 0x00, // Flags
        0x00, 0x14, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        // Payload
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
fn from_bytes_control_invalid_priority() {
    // ControlMessage with invalid priority
    let input = vec![
        0x83, 0x00, // Flags
        0x00, 0x04, // Length
        0x00, 0x02, // Tunnel ID
        0x00, 0x03, // Session ID
        0x00, 0x04, // Ns
        0x00, 0x05, // Nr
        0xde, 0xad, // Payload
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
        Err("Control message with forbidden Priority bit encountered")
    );
}
