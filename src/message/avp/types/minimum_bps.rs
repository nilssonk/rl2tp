use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MinimumBps {
    pub value: u32,
}

impl MinimumBps {
    const ATTRIBUTE_TYPE: u16 = 16;
    const LENGTH: usize = 4;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete MinimumBps AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for MinimumBps {
    #[inline]
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl QueryableAVP for MinimumBps {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for MinimumBps {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_u32_be_unchecked(self.value);
    }
}
