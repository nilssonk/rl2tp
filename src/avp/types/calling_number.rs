use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct CallingNumber {
    pub value: String,
}

impl CallingNumber {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete CallingNumber AVP encountered");
        }

        let value = std::str::from_utf8(input)
            .map_err(|_| "Parsing CallingNumber advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}
