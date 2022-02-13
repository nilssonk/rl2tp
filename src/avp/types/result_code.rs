use crate::common::{read_u16_be_unchecked, ResultStr};

mod code;
pub use code::*;

mod error;
pub use error::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ResultCode {
    pub code: CodeValue,
    pub error: Option<Error>,
    pub error_message: Option<String>,
}

impl ResultCode {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 2 {
            return Err("Incomplete ResultCode AVP payload encountered");
        }

        let code_raw = unsafe { read_u16_be_unchecked(input) };
        let code = CodeValue::from(code_raw);

        let mut maybe_error: Option<Error> = None;
        let mut maybe_error_message: Option<String> = None;
        if input.len() >= 4 {
            let error_raw = unsafe { read_u16_be_unchecked(&input[2..]) };
            maybe_error = error_raw.try_into().ok();
            if maybe_error.is_none() {
                return Err("Invalid ResultCode error encountered");
            }

            if input.len() > 4 {
                maybe_error_message = match std::str::from_utf8(&input[4..]) {
                    Ok(s) => Some(s.to_owned()),
                    Err(_) => return Err("Invalid ResultCode error message encountered"),
                }
            }
        }

        Ok(Self {
            code,
            error: maybe_error,
            error_message: maybe_error_message,
        })
    }
}
