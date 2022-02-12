use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct ProxyAuthenChallenge {
    pub data: Vec<u8>,
}

impl ProxyAuthenChallenge {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete ProxyAuthenChallenge AVP encountered");
        }

        Ok(Self {
            data: input.to_owned(),
        })
    }
}
