use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignedSessionId {
    pub value: u16,
}

impl AssignedSessionId {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 2 {
            return Err("Incomplete AssignedSessionId AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u16> for AssignedSessionId {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl QueryableAVP for AssignedSessionId {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
