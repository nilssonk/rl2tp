use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

const G_LENGTH: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RandomVector {
    pub value: [u8; G_LENGTH as usize],
}

impl RandomVector {
    const ATTRIBUTE_TYPE: u16 = 36;
    const LENGTH: usize = G_LENGTH;

    #[inline]
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete Random Vector AVP payload encountered");
        }

        Ok(Self {
            value: unsafe { reader.peek_bytes(4)?.try_into().unwrap_unchecked() },
        })
    }
}

impl QueryableAVP for RandomVector {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for RandomVector {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(&self.value);
    }
}
