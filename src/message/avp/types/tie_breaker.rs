use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TieBreaker {
    pub value: u64,
}

impl TieBreaker {
    const ATTRIBUTE_TYPE: u16 = 5;
    const LENGTH: usize = 8;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete TieBreaker AVP encountered");
        }

        let value = unsafe { reader.read_u64_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u64> for TieBreaker {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl QueryableAVP for TieBreaker {
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for TieBreaker {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_u64_be_unchecked(self.value);
    }
}