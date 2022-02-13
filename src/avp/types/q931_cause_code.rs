use crate::common::{read_u16_be_unchecked, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct Q931CauseCode {
    pub cause_code: u16,
    pub cause_msg: u8,
    pub advisory: Option<String>,
}

impl Q931CauseCode {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 3 {
            return Err("Incomplete Q931CauseCode AVP encountered");
        }

        let cause_code = unsafe { read_u16_be_unchecked(&input[..2]) };
        let cause_msg = input[2];

        let advisory = if input.len() > 3 {
            Some(
                std::str::from_utf8(&input[3..])
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
