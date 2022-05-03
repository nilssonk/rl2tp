use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

const G_LENGTH: u16 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomVector {
    pub data: [u8; G_LENGTH as usize],
}

impl RandomVector {
    const LENGTH: u16 = G_LENGTH;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete Random Vector AVP payload encountered");
        }

        Ok(Self {
            data: unsafe { reader.peek_bytes(4)?.try_into().unwrap_unchecked() },
        })
    }
}

impl QueryableAVP for RandomVector {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for RandomVector {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
