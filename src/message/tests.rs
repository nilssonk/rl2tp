use crate::message::*;

#[test]
fn from_bytes() {
    // Data message with invalid version
    {
        let input = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff];
        let m = Message::from_bytes(
            &input,
            ValidateReserved::No,
            ValidateVersion::Yes,
            ValidateUnused::No,
        );
        assert_eq!(m, Err("Invalid version encountered"));
    }
    // Data message with valid version
    {
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
    // ControlMessage with invalid priority
    {
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
    // ControlMessage with valid priority
    {
        let input = vec![
            0x13, 0x00, // Flags
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
            Ok(Message::Control(ControlMessage {
                length: 4,
                tunnel_id: 2,
                session_id: 3,
                ns: 4,
                nr: 5,
                data: &input[12..]
            }))
        );
    }
}
