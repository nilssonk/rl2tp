use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaximumBps {
    pub value: u32,
}

impl MaximumBps {
    const LENGTH: u16 = 4;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete MaximumBps AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for MaximumBps {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl QueryableAVP for MaximumBps {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for MaximumBps {
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
