use crate::common::{Reader, Writer};
use crate::message::flags::{Flags, MessageFlagType};
use crate::message::*;

#[derive(Debug, Eq, PartialEq)]
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
    pub(crate) fn try_read<'b>(flags: Flags, reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
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

        Ok(DataMessage {
            is_prioritized: false,
            length: maybe_length,
            tunnel_id,
            session_id,
            ns_nr: maybe_ns_nr,
            offset: None,
            data,
        })
    }

    pub(crate) unsafe fn write(&self, protocol_version: u8, writer: &mut impl Writer) {
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
