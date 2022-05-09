use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

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
    const FIXED_LENGTH: usize = 2;
    const ERROR_LENGTH: usize = 2;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::FIXED_LENGTH {
            return Err("Incomplete ResultCode AVP payload encountered");
        }

        let code_raw = unsafe { reader.read_u16_be_unchecked() };
        let code = CodeValue::from(code_raw);

        let mut maybe_error: Option<Error> = None;
        let mut maybe_error_message: Option<String> = None;
        if reader.len() >= Self::ERROR_LENGTH {
            let error_raw = unsafe { reader.read_u16_be_unchecked() };
            maybe_error = error_raw.try_into().ok();
            if maybe_error.is_none() {
                return Err("Invalid ResultCode error encountered");
            }

            if !reader.is_empty() {
                maybe_error_message = match std::str::from_utf8(reader.peek_bytes(reader.len())?) {
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

impl QueryableAVP for ResultCode {
    fn get_length(&self) -> usize {
        let mut length = Self::FIXED_LENGTH;

        if self.error.is_some() {
            length += Self::ERROR_LENGTH;

            if let Some(value) = &self.error_message {
                length += value.len()
            }
        }

        length
    }
}

impl WritableAVP for ResultCode {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
