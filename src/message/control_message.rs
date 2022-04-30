use crate::avp::AVP;
use crate::common::Writer;
use crate::message::flags::{Flags, MessageFlagType};

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
    fn get_dynamic_length(&self) -> u16 {
        self.avps.iter().map(|avp| avp.get_length()).sum::<u16>()
    }

    /// # Summary
    /// Write a `ControlMessage` using a mutable `Writer`.
    /// # Safety
    /// This function is marked as unsafe because the `Writer` trait offers no error handling mechanism.
    pub unsafe fn write(&self, protocol_version: u8, writer: &mut impl Writer) {
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
