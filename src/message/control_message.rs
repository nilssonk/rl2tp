use crate::common::{Reader, ResultStr, Writer};
use crate::message::flags::{Flags, MessageFlagType};
use crate::message::*;
use avp::AVP;

/// # Summary
/// A `ControlMessage` is a representation of an L2TP control message which is the primary link control mechanism of the protocol.
/// 
/// # Data members
/// * `length` - The payload length field.
/// * `tunnel_id` - The tunnel identifier field.
/// * `session_id` - The session identifier field.
/// * `ns` - The NS field.
/// * `nr` - The NR field.
/// * `avps` - A collection of Attribute Value Pairs constituting the payload of this message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlMessage {
    pub length: u16,
    pub tunnel_id: u16,
    pub session_id: u16,
    pub ns: u16,
    pub nr: u16,
    pub avps: Vec<AVP>,
}

impl ControlMessage {
    #[inline]
    pub(crate) fn try_read<'a, 'b>(
        flags: Flags,
        validation_options: ValidationOptions,
        reader: &'b mut impl Reader<'a>,
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

        Ok(ControlMessage {
            length,
            tunnel_id,
            session_id,
            ns,
            nr,
            avps,
        })
    }

    pub(crate) unsafe fn write(&self, protocol_version: u8, writer: &mut impl Writer) {
        let start_position = writer.len();
        let flags = Flags::new(
            MessageFlagType::Control,
            true,
            true,
            false,
            false,
            protocol_version,
        );
        flags.write(writer);

        // Save length field position
        let length_position = writer.len();

        // Dummy octets to be overwritten
        writer.write_bytes_unchecked(&[0, 0]);

        // Write rest of header
        writer.write_u16_be_unchecked(self.tunnel_id);
        writer.write_u16_be_unchecked(self.session_id);
        writer.write_u16_be_unchecked(self.ns);
        writer.write_u16_be_unchecked(self.nr);

        // Write payload
        for avp in self.avps.iter() {
            avp.write(writer);
        }

        // Get total length
        let end_position = writer.len();
        let length = end_position - start_position;

        // Overwrite dummy octets
        assert!(length <= u16::MAX as usize);
        writer.write_bytes_unchecked_at(&(length as u16).to_be_bytes(), length_position);
    }
}
