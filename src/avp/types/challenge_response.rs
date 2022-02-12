use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeResponse {
    pub data: [u8; 16],
}

impl ChallengeResponse {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 16 {
            return Err("Incomplete ChallengeResponse AVP encountered");
        }

        Ok(Self {
            data: unsafe { input[..16].try_into().unwrap_unchecked() },
        })
    }
}
