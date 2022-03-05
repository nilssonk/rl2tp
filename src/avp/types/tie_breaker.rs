use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TieBreaker {
    pub value: u64,
}

impl TieBreaker {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 8 {
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
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
