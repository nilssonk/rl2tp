use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct LastReceivedLcpConfReq {
    pub data: Vec<u8>,
}

impl LastReceivedLcpConfReq {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete LastReceivedLcpConfReq AVP encountered");
        }

        Ok(Self {
            data: input.to_owned(),
        })
    }
}
