///! Types and implementations related to L2TP protocol messages.

#[cfg(test)]
mod tests;

pub mod avp;

mod control_message;
pub use control_message::ControlMessage;

mod data_message;
pub use data_message::DataMessage;

mod flags;
use flags::{Flags, MessageFlagType};

use crate::common::{Reader, ResultStr, Writer};

/// # Summary
/// A `Message` is a representation of an L2TP protocol message. It can be either a `DataMessage`
/// or a `ControlMessage` and constitutes the outermost container for the protocol.
#[derive(Debug, Eq, PartialEq)]
pub enum Message<'a> {
    Control(ControlMessage),
    Data(DataMessage<'a>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValidateReserved {
    Yes,
    No,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValidateVersion {
    Yes,
    No,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValidateUnused {
    Yes,
    No,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationOptions {
    reserved: ValidateReserved,
    version: ValidateVersion,
    unused: ValidateUnused,
}

impl<'a> Message<'a> {
    const PROTOCOL_VERSION: u8 = 2;

    /// # Summary
    /// Attempt to read a `Message` using a `Reader`. User-supplied `ValidationOptions` offer a way to ignore certain protocol mandates.
    pub fn try_read<'b>(
        reader: &'b mut impl Reader<'a>,
        validation_options: ValidationOptions,
    ) -> ResultStr<Self> {
        let flags = Flags::read(reader)?;

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
            MessageFlagType::Data => Ok(Message::Data(DataMessage::try_read(flags, reader)?)),
            MessageFlagType::Control => Ok(Message::Control(ControlMessage::try_read(
                flags,
                validation_options,
                reader,
            )?)),
        }
    }

    /// # Summary
    /// Write a `Message` using a mutable `Writer`.
    /// 
    /// # Safety
    /// This function is marked as unsafe because the `Writer` trait offers no error handling mechanism.
    #[inline]
    pub unsafe fn write(&self, writer: &mut impl Writer) {
        match self {
            Message::Control(control) => control.write(Self::PROTOCOL_VERSION, writer),
            Message::Data(data) => data.write(Self::PROTOCOL_VERSION, writer),
        }
    }
}
