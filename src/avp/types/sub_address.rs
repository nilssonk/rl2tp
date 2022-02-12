use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct SubAddress {
    pub value: String,
}

impl SubAddress {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete SubAddress AVP encountered");
        }

        let value = std::str::from_utf8(input)
            .map_err(|_| "Parsing SubAddress advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}
