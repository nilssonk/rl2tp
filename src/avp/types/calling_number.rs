use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct CallingNumber {
    pub value: String,
}

impl CallingNumber {
    pub fn try_read<'a>(reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete CallingNumber AVP encountered");
        }

        let value = std::str::from_utf8(reader.peek_bytes(reader.len())?)
            .map_err(|_| "Parsing CallingNumber advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}
