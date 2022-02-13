use crate::common::ResultStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProtocolVersion {
    pub version: u8,
    pub revision: u8,
}

impl ProtocolVersion {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 2 {
            return Err("Incomplete ProtocolVersion AVP encountered");
        }

        Ok(Self {
            version: input[0],
            revision: input[1],
        })
    }
}
