use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

const G_LENGTH: u16 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicalChannelId {
    pub data: [u8; G_LENGTH as usize],
}

impl PhysicalChannelId {
    const LENGTH: u16 = G_LENGTH;

    pub fn try_read<'a>(reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete PhysicalChannelId payload encountered");
        }

        Ok(Self {
            data: unsafe {
                reader
                    .peek_bytes(Self::LENGTH as usize)?
                    .try_into()
                    .unwrap_unchecked()
            },
        })
    }
}

impl QueryableAVP for PhysicalChannelId {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}
