use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TieBreaker {
    pub value: u64,
}

impl TieBreaker {
    const LENGTH: u16 = 8;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
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
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, 0)
    }
}

impl WritableAVP for TieBreaker {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
