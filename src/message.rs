#[cfg(test)]
mod tests;

mod control_message;
pub use control_message::ControlMessage;

mod data_message;
pub use data_message::DataMessage;

mod flags;
use flags::{Flags, MessageFlagType};

use crate::avp::AVP;
use crate::common::{Reader, ResultStr, Writer};

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

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationOptions {
    reserved: ValidateReserved,
    version: ValidateVersion,
    unused: ValidateUnused,
}

impl<'a> Message<'a> {
    const PROTOCOL_VERSION: u8 = 2;

    /// # Summary
    /// Attempt to read a `Message` using a `Reader`. User-supplied ValidationOptions offer a way to ignore certain protocol mandates.
    pub fn try_read(
        mut reader: Box<dyn Reader<'a> + 'a>,
        validation_options: ValidationOptions,
    ) -> ResultStr<Self> {
        let flags = Flags::read(reader.as_mut())?;

        if let ValidateVersion::Yes = validation_options.version {
            let version = flags.get_version();
            if version != Self::PROTOCOL_VERSION {
                return Err("Invalid version encountered");
            }
        }

        if let ValidateReserved::Yes = validation_options.reserved {
            if !flags.reserved_bits_ok() {
                return Err("Invalid reserved bits encountered");
            }
        }

        match flags.get_type() {
            MessageFlagType::Data => Self::try_read_data_message(flags, reader),
            MessageFlagType::Control => {
                Self::try_read_control_message(flags, validation_options, reader)
            }
        }
    }

    /// # Summary
    /// Write a `Message` using a mutable `Writer`.
    /// # Safety
    /// This function is marked as unsafe because the `Writer` trait offers no error handling mechanism.
    pub unsafe fn write(&self, writer: &mut dyn Writer) {
        match self {
            Message::Control(control) => control.write(Self::PROTOCOL_VERSION, writer),
            Message::Data(data) => data.write(Self::PROTOCOL_VERSION, writer),
        }
    }

    fn try_read_data_message(
        flags: Flags,
        mut reader: Box<dyn Reader<'a> + 'a>,
    ) -> ResultStr<Self> {
        let mut minimal_length = 4;
        if flags.has_length() {
            minimal_length += 2;
        }
        if flags.has_ns_nr() {
            minimal_length += 4;
        }
        if flags.has_offset() {
            minimal_length += 4;
        }
        if reader.len() < minimal_length {
            return Err("Incomplete data message header encountered");
        }

        let maybe_length = if flags.has_length() {
            let length = unsafe { reader.read_u16_be_unchecked() };
            Some(length)
        } else {
            None
        };

        let tunnel_id = unsafe { reader.read_u16_be_unchecked() };
        let session_id = unsafe { reader.read_u16_be_unchecked() };

        let maybe_ns_nr = if flags.has_ns_nr() {
            let ns = unsafe { reader.read_u16_be_unchecked() };
            let nr = unsafe { reader.read_u16_be_unchecked() };
            Some((ns, nr))
        } else {
            None
        };

        if flags.has_offset() {
            let offset_size = unsafe { reader.read_u16_be_unchecked() as usize };
            if reader.len() < offset_size {
                return Err("Invalid offset size encountered");
            }
            reader.skip_bytes(offset_size);
        }

        let data = reader.peek_bytes(reader.len())?;

        Ok(Message::Data(DataMessage {
            is_prioritized: false,
            length: maybe_length,
            tunnel_id,
            session_id,
            ns_nr: maybe_ns_nr,
            offset: None,
            data,
        }))
    }

    fn try_read_control_message(
        flags: Flags,
        validation_options: ValidationOptions,
        mut reader: Box<dyn Reader<'a> + 'a>,
    ) -> ResultStr<Self> {
        if let ValidateUnused::Yes = validation_options.unused {
            if flags.is_prioritized() {
                return Err("Control message with forbidden Priority bit encountered");
            }

            if flags.has_offset() {
                return Err("Control message with forbidden Offset fields encountered");
            }
        }

        if !flags.has_length() {
            return Err("Control message without required Length field encountered");
        }
        if !flags.has_ns_nr() {
            return Err("Control message without required Sequence fields encountered");
        }

        const FIXED_LENGTH_MINUS_FLAGS: usize = 10;
        if reader.len() < FIXED_LENGTH_MINUS_FLAGS {
            return Err("Incomplete control message header encountered");
        }

        let length = unsafe { reader.read_u16_be_unchecked() };
        let tunnel_id = unsafe { reader.read_u16_be_unchecked() };
        let session_id = unsafe { reader.read_u16_be_unchecked() };
        let ns = unsafe { reader.read_u16_be_unchecked() };
        let nr = unsafe { reader.read_u16_be_unchecked() };

        let avp_and_err = AVP::try_read_greedy(reader);

        if let Some(first) = avp_and_err.first() {
            match first {
                Ok(AVP::MessageType(_)) => (),
                _ => return Err("First AVP is not a MessageType AVP"),
            }
        }

        if avp_and_err.iter().any(|x| {
            println!("{:?}", x);
            x.is_err()
        }) {
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
