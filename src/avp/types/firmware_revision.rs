use crate::common::{read_u16_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareRevision {
    pub value: u16,
}

impl FirmwareRevision {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 2 {
            return Err("Incomplete FirmwareRevision AVP encountered");
        }

        let value = unsafe { read_u16_be_unchecked(input) };
        Ok(Self { value })
    }
}

impl From<u16> for FirmwareRevision {
    fn from(value: u16) -> Self {
        Self { value }
    }
}
