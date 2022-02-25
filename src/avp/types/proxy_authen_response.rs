use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct ProxyAuthenResponse {
    pub data: Vec<u8>,
}

impl ProxyAuthenResponse {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete ProxyAuthenResponse AVP encountered");
        }

        Ok(Self {
            data: input.to_owned(),
        })
    }
}
