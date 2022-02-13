use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Challenge {
    pub data: Vec<u8>,
}

impl Challenge {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete Challenge AVP encountered");
        }

        Ok(Self {
            data: input.to_owned(),
        })
    }
}
