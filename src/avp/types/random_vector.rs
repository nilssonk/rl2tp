use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomVector {
    pub data: [u8; 4],
}

impl RandomVector {
    pub fn try_read<'a>(reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 4 {
            return Err("Incomplete Random Vector AVP payload encountered");
        }

        Ok(Self {
            data: unsafe { reader.peek_bytes(4)?.try_into().unwrap_unchecked() },
        })
    }
}

impl QueryableAVP for RandomVector {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
