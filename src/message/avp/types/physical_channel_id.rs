use crate::common::{Reader, ResultStr, Writer};
use crate::message::avp::{QueryableAVP, WritableAVP};

const G_PHYSICAL_CHANNEL_ID_LENGTH: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalChannelId {
    pub value: [u8; G_PHYSICAL_CHANNEL_ID_LENGTH],
}

impl PhysicalChannelId {
    const ATTRIBUTE_TYPE: u16 = 25;
    const LENGTH: usize = G_PHYSICAL_CHANNEL_ID_LENGTH;

    #[inline]
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete PhysicalChannelId payload encountered");
        }

        let value = unsafe {
            reader
                .peek_bytes(Self::LENGTH)?
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
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(&self.value);
    }
}
