use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimumBps {
    pub value: u32,
}

impl MinimumBps {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 4 {
            return Err("Incomplete MinimumBps AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for MinimumBps {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl QueryableAVP for MinimumBps {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
