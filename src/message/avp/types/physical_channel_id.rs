use crate::common::{DecodeError, DecodeResult, Reader, Writer};
use crate::message::avp::{QueryableAVP, WritableAVP};
use core::borrow::Borrow;

const G_PHYSICAL_CHANNEL_ID_LENGTH: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalChannelId {
    pub value: [u8; G_PHYSICAL_CHANNEL_ID_LENGTH],
}

impl PhysicalChannelId {
    const ATTRIBUTE_TYPE: u16 = 25;
    const LENGTH: usize = G_PHYSICAL_CHANNEL_ID_LENGTH;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        let value = unsafe {
            reader
                .bytes(Self::LENGTH)
                .ok_or(DecodeError::AVPReadError(Self::ATTRIBUTE_TYPE))?
                .borrow()
                .try_into()
                .unwrap_unchecked()
        };

        Ok(Self { value })
    }
}

impl From<[u8; G_PHYSICAL_CHANNEL_ID_LENGTH]> for PhysicalChannelId {
    fn from(value: [u8; G_PHYSICAL_CHANNEL_ID_LENGTH]) -> Self {
        Self { value }
    }
}

impl From<PhysicalChannelId> for [u8; G_PHYSICAL_CHANNEL_ID_LENGTH] {
    fn from(value: PhysicalChannelId) -> Self {
        value.value
    }
}

impl QueryableAVP for PhysicalChannelId {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for PhysicalChannelId {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_bytes(&self.value);
    }
}
