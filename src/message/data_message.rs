use crate::common::{Reader, Writer};
use crate::message::flags::{Flags, MessageFlagType};
use crate::message::*;
use core::borrow::Borrow;

/// # Summary
/// A `DataMessage` is a representation of an L2TP data message which is the primary data transfer mechanism of the protocol.
///
/// # Lifetimes
/// * `'a` - The lifetime of the borrowed slice of bytes contained in `data`.
///
/// # Data members
/// * `is_prioritized` - Indicates whether this is a prioritized message.
/// * `length` - The optional data length field.
/// * `tunnel_id` - The tunnel identifier.
/// * `session_id` - The session identifier.
/// * `ns_nr` - The optional NS/NR field.
/// * `offset` - The optional data offset field.
/// * `data` - A borrowed slice of data belonging to this data message.
#[derive(Debug, Eq, PartialEq)]
pub struct DataMessage<T> {
    pub is_prioritized: bool,
    pub length: Option<u16>,
    pub tunnel_id: u16,
    pub session_id: u16,
    pub ns_nr: Option<(u16, u16)>,
    pub offset: Option<u16>,
    pub data: T,
}

impl<T> DataMessage<T>
where
    T: Borrow<[u8]>,
{
    #[inline]
    pub(crate) fn try_read(flags: Flags, reader: &mut impl Reader<T>) -> ResultStr<Self> {
        let mut minimal_length_minus_flags = 4;
        if flags.has_length() {
            minimal_length_minus_flags += 2;
        }
        if flags.has_ns_nr() {
            minimal_length_minus_flags += 4;
        }
        if flags.has_offset() {
            minimal_length_minus_flags += 4;
        }
        if reader.len() < minimal_length_minus_flags {
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

        let payload_length;
        if let Some(length) = maybe_length {
            let minimal_length = minimal_length_minus_flags + 2;
            if length as usize > reader.len() + minimal_length {
                return Err("Incomplete data message payload encountered");
            }
            payload_length = length as usize;
        } else {
            payload_length = reader.len();
        }

        if reader.is_empty() {
            return Err("Empty data message payload encountered");
        }

        let data = reader.bytes(payload_length)?;

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

    #[inline]
    pub(crate) fn write(&self, protocol_version: u8, writer: &mut impl Writer) {
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
            writer.write_u16_be(length);
        }

        writer.write_u16_be(self.tunnel_id);
        writer.write_u16_be(self.session_id);
        if let Some((ns, nr)) = self.ns_nr {
            writer.write_u16_be(ns);
            writer.write_u16_be(nr);
        }
        if let Some(offset) = self.offset {
            writer.write_u16_be(offset);
        }
        writer.write_bytes(self.data.borrow());
    }
}
