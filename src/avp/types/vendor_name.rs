use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct VendorName {
    pub data: Vec<u8>,
}

impl VendorName {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete VendorName AVP encountered");
        }

        Ok(Self {
            data: input.to_owned(),
        })
    }
}
