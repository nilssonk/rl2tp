use crate::common::ResultStr;

#[derive(Clone, Debug, PartialEq)]
pub struct CalledNumber {
    pub value: String,
}

impl CalledNumber {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.is_empty() {
            return Err("Incomplete CalledNumber AVP encountered");
        }

        let value = std::str::from_utf8(input)
            .map_err(|_| "Parsing CalledNumber advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}
