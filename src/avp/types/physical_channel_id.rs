use crate::common::ResultStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicalChannelId {
    pub data: [u8; 4],
}

impl PhysicalChannelId {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 4 {
            return Err("Incomplete PhysicalChannelId payload encountered");
        }

        Ok(Self {
            data: unsafe { input[..4].try_into().unwrap_unchecked() },
        })
    }
}
