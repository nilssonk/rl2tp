use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct Q931CauseCode {
    pub cause_code: u16,
    pub cause_msg: u8,
    pub advisory: Option<String>,
}

impl Q931CauseCode {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 3 {
            return Err("Incomplete Q931CauseCode AVP encountered");
        }

        let cause_code = unsafe { reader.read_u16_be_unchecked() };
        let cause_msg = unsafe { reader.read_u8_unchecked() };

        let advisory = if !reader.is_empty() {
            Some(
                std::str::from_utf8(reader.peek_bytes(reader.len())?)
                    .map_err(|_| "Parsing Q931CauseCode advisory message failed")?
                    .to_owned(),
            )
        } else {
            None
        };

        Ok(Self {
            cause_code,
            cause_msg,
            advisory,
        })
    }
}
