use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CallSerialNumber {
    pub value: u32,
}

impl CallSerialNumber {
    const ATTRIBUTE_TYPE: u16 = 15;
    const LENGTH: usize = 4;

    #[inline]
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete CallSerialNumber AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for CallSerialNumber {
    #[inline]
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl QueryableAVP for CallSerialNumber {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for CallSerialNumber {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_u32_be_unchecked(self.value);
    }
}
