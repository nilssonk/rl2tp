use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct PrivateGroupId {
    pub data: Vec<u8>,
}

impl PrivateGroupId {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete PrivateGroupId AVP encountered");
        }

        Ok(Self {
            data: input.to_owned(),
        })
    }
}
