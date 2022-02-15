use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeResponse {
    pub data: [u8; 16],
}

impl ChallengeResponse {
    pub fn try_read<'a>(reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 16 {
            return Err("Incomplete ChallengeResponse AVP encountered");
        }

        Ok(Self {
            data: unsafe { reader.peek_bytes(16)?.try_into().unwrap_unchecked() },
        })
    }
}
