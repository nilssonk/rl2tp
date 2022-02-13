#[cfg(test)]
mod tests;

mod flags;

use crate::avp::AVP;
use flags::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ControlMessage {
    pub length: u16,
    pub tunnel_id: u16,
    pub session_id: u16,
    pub ns: u16,
    pub nr: u16,
    pub avps: Vec<AVP>,
}

#[derive(Debug, PartialEq)]
pub struct DataMessage<'a> {
    pub length: Option<u16>,
    pub tunnel_id: u16,
    pub session_id: u16,
    pub ns_nr: Option<(u16, u16)>,
    pub offset: Option<(u16, u16)>,
    pub data: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub enum Message<'a> {
    Control(ControlMessage),
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

                let avp_and_err = AVP::try_from_bytes_greedy(&input[offset..length as usize]);

                if let Some(first) = avp_and_err.first() {
                    match first {
                        Ok(AVP::MessageType(_)) => (),
                        _ => return Err("First AVP is not a MessageType AVP"),
                    }
                }

                if avp_and_err.iter().any(|x| x.is_err()) {
                    // @TODO: Better error reporting
                    // @TODO: Allow errors?
                    return Err("AVP errors detected in control message");
                }

                let avps = avp_and_err.into_iter().filter_map(|x| x.ok()).collect();

                Ok(Message::Control(ControlMessage {
                    length,
                    tunnel_id,
                    session_id,
                    ns,
                    nr,
                    avps,
                }))
            }
        }
    }
}
