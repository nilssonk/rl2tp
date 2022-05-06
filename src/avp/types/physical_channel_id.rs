use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

const G_LENGTH: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicalChannelId {
    pub value: [u8; G_LENGTH],
}

impl PhysicalChannelId {
    const ATTRIBUTE_TYPE: u16 = 25;
    const LENGTH: usize = G_LENGTH;

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

impl QueryableAVP for PhysicalChannelId {
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
