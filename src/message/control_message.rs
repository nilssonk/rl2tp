use crate::avp::AVP;
use crate::common::{Reader, ResultStr, Writer};
use crate::message::flags::{Flags, MessageFlagType};
use crate::message::*;

#[derive(Clone, Debug, PartialEq)]
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
    fn get_dynamic_length(&self) -> u16 {
        self.avps.iter().map(|avp| avp.get_length()).sum::<u16>()
    }

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
        let flags = Flags::new(
            MessageFlagType::Control,
            true,
            true,
            false,
            false,
            protocol_version,
        );
        flags.write(writer);

        const FIXED_LENGTH: u16 = 12;
        let dynamic_length = self.get_dynamic_length();
        writer.write_u16_be_unchecked(FIXED_LENGTH + dynamic_length);
        writer.write_u16_be_unchecked(self.tunnel_id);
        writer.write_u16_be_unchecked(self.session_id);
        writer.write_u16_be_unchecked(self.ns);
        writer.write_u16_be_unchecked(self.nr);
        for avp in self.avps.iter() {
            avp.write(writer);
        }
    }
}
