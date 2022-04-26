use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

const G_LENGTH: u16 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicalChannelId {
    pub value: [u8; G_LENGTH as usize],
}

impl PhysicalChannelId {
    const LENGTH: u16 = G_LENGTH;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete PhysicalChannelId payload encountered");
        }

        Ok(Self {
            value: unsafe {
                reader
                    .peek_bytes(Self::LENGTH as usize)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}

impl QueryableAVP for PhysicalChannelId {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, 0)
    }
}

impl WritableAVP for PhysicalChannelId {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
