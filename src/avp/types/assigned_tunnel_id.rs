use crate::common::{read_u16_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignedTunnelId {
    pub value: u16,
}

impl AssignedTunnelId {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 2 {
            return Err("Incomplete AssignedTunnelId AVP encountered");
        }

        let value = unsafe { read_u16_be_unchecked(input) };
        Ok(Self { value })
    }
}

impl From<u16> for AssignedTunnelId {
    fn from(value: u16) -> Self {
        Self { value }
    }
}
