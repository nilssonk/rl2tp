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
