use crate::common::Writer;
use crate::message::flags::{Flags, MessageFlagType};

#[derive(Debug, PartialEq)]
pub struct DataMessage<'a> {
    pub is_prioritized: bool,
    pub length: Option<u16>,
    pub tunnel_id: u16,
    pub session_id: u16,
    pub ns_nr: Option<(u16, u16)>,
    pub offset: Option<u16>,
    pub data: &'a [u8],
}

impl<'a> DataMessage<'a> {
    /// # Summary
    /// Write a `DataMessage` using a mutable `Writer`.
    /// # Safety
    /// This function is marked as unsafe because the `Writer` trait offers no error handling mechanism.
    pub unsafe fn write(&self, protocol_version: u8, writer: &mut impl Writer) {
        let flags = Flags::new(
            MessageFlagType::Data,
            self.length.is_some(),
            self.ns_nr.is_some(),
            self.offset.is_some(),
            self.is_prioritized,
            protocol_version,
        );
        flags.write(writer);

        if let Some(length) = self.length {
            writer.write_u16_be_unchecked(length);
        }

        writer.write_u16_be_unchecked(self.tunnel_id);
        writer.write_u16_be_unchecked(self.session_id);
        if let Some((ns, nr)) = self.ns_nr {
            writer.write_u16_be_unchecked(ns);
            writer.write_u16_be_unchecked(nr);
        }
        if let Some(offset) = self.offset {
            writer.write_u16_be_unchecked(offset);
        }
        writer.write_bytes_unchecked(self.data);
    }
}
