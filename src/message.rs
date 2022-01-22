use crate::flags::*;

#[derive(Debug, PartialEq)]
pub struct ControlMessage<'a> {
    length: u16,
    tunnel_id: u16,
    session_id: u16,
    ns: u16,
    nr: u16,
    data: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct DataMessage<'a> {
    length: Option<u16>,
    tunnel_id: u16,
    session_id: u16,
    ns_nr: Option<(u16, u16)>,
    offset: Option<(u16, u16)>,
    data: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub enum Message<'a> {
    Control(ControlMessage<'a>),
    Data(DataMessage<'a>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValidateReserved {
    Yes,
    No,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValidateVersion {
    Yes,
    No,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValidateUnused {
    Yes,
    No,
}

impl<'a> Message<'a> {
    pub fn from_bytes(
        input: &'a [u8],
        validate_reserved: ValidateReserved,
        validate_version: ValidateVersion,
        validate_unused: ValidateUnused,
    ) -> Result<Self, &'static str> {
        let mut offset = 0;
        let flags = Flags::from_bytes(&input[offset..offset + 2])?;
        offset += 2;

        if let ValidateVersion::Yes = validate_version {
            let version = flags.get_version();
            if version != 2 {
                return Err("Invalid version encountered");
            }
        }

        if let ValidateReserved::Yes = validate_reserved {
            if !flags.reserved_bits_ok() {
                return Err("Invalid reserved bits encountered");
            }
        }

        let read_length = |off: &mut usize| {
            let remaining = &input[*off..];
            if remaining.len() < 2 {
                return Err("Incomplete header encountered");
            }

            // We have already performed length checks, so the field is guaranteed to be complete.
            let length =
                unsafe { u16::from_be_bytes(input[*off..*off + 2].try_into().unwrap_unchecked()) };
            *off += 2;

            Ok(length)
        };

        let read_pair = |off: &mut usize| {
            let remaining = &input[*off..];
            if remaining.len() < 4 {
                return Err("Incomplete header encountered");
            }

            // We have already performed length checks, so the fields are guaranteed to be complete.
            let a =
                unsafe { u16::from_be_bytes(input[*off..*off + 2].try_into().unwrap_unchecked()) };
            *off += 2;
            let b =
                unsafe { u16::from_be_bytes(input[*off..*off + 2].try_into().unwrap_unchecked()) };
            *off += 2;

            Ok((a, b))
        };

        match flags.get_type() {
            MessageFlagType::Data => {
                let maybe_length = if flags.has_length() {
                    let length = read_length(&mut offset)?;
                    Some(length)
                } else {
                    None
                };

                let (tunnel_id, session_id) = read_pair(&mut offset)?;

                let maybe_ns_nr = if flags.has_ns_nr() {
                    let ns_nr = read_pair(&mut offset)?;
                    Some(ns_nr)
                } else {
                    None
                };

                let maybe_offset_size_pad = if flags.has_offset_size() {
                    let offset_size_pad = read_pair(&mut offset)?;
                    Some(offset_size_pad)
                } else {
                    None
                };

                Ok(Message::Data(DataMessage {
                    length: maybe_length,
                    tunnel_id,
                    session_id,
                    ns_nr: maybe_ns_nr,
                    offset: maybe_offset_size_pad,
                    data: &input[offset..],
                }))
            }
            MessageFlagType::Control => {
                if let ValidateUnused::Yes = validate_unused {
                    if flags.is_prioritized() {
                        return Err("Control message with forbidden Priority bit encountered");
                    }

                    if flags.has_offset_size() {
                        return Err("Control message with forbidden Offset fields encountered");
                    }
                }

                if !flags.has_length() {
                    return Err("Control message without required Length field encountered");
                }
                let length = read_length(&mut offset)?;

                let (tunnel_id, session_id) = read_pair(&mut offset)?;

                if !flags.has_ns_nr() {
                    return Err("Control message without required Sequence fields encountered");
                }
                let (ns, nr) = read_pair(&mut offset)?;

                Ok(Message::Control(ControlMessage {
                    length,
                    tunnel_id,
                    session_id,
                    ns,
                    nr,
                    data: &input[offset..],
                }))
            }
        }
    }
}
