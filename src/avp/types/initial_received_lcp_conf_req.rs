use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct InitialReceivedLcpConfReq {
    pub data: Vec<u8>,
}

impl InitialReceivedLcpConfReq {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete InitialReceivedLcpConfReq AVP encountered");
        }

        Ok(Self {
            data: input.to_owned(),
        })
    }
}
