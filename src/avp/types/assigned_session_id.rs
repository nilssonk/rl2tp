use crate::common::{read_u16_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignedSessionId {
    pub value: u16,
}

impl AssignedSessionId {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 2 {
            return Err("Incomplete AssignedSessionId AVP encountered");
        }

        let value = unsafe { read_u16_be_unchecked(input) };
        Ok(Self { value })
    }
}

impl From<u16> for AssignedSessionId {
    fn from(value: u16) -> Self {
        Self { value }
    }
}
