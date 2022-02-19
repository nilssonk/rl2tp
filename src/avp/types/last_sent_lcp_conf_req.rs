use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct LastSentLcpConfReq {
    pub data: Vec<u8>,
}

impl LastSentLcpConfReq {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete LastSentLcpConfReq AVP encountered");
        }

        Ok(Self {
            data: input.to_owned(),
        })
    }
}
