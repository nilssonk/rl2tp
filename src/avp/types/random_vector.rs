use crate::common::ResultStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomVector {
    pub data: [u8; 4],
}

impl RandomVector {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 4 {
            return Err("Incomplete Random Vector AVP payload encountered");
        }

        Ok(Self {
            data: unsafe { input[..4].try_into().unwrap_unchecked() },
        })
    }
}
