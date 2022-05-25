use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

mod code;
pub use code::*;

mod error;
pub use error::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResultCode {
    pub code: CodeValue,
    pub error: Option<Error>,
}

impl ResultCode {
    const ATTRIBUTE_TYPE: u16 = 1;
    const FIXED_LENGTH: usize = 2;
    const ERROR_LENGTH: usize = 2;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::FIXED_LENGTH {
            return Err("Incomplete ResultCode AVP payload encountered");
        }

        let code_raw = unsafe { reader.read_u16_be_unchecked() };
        let code = CodeValue::from(code_raw);

        let error = if reader.len() >= Self::ERROR_LENGTH as usize {
            Some(unsafe { Error::try_read(reader)? })
        } else {
            None
        };

        Ok(Self { code, error })
    }
}

impl QueryableAVP for ResultCode {
    fn get_length(&self) -> usize {
        let mut length = Self::FIXED_LENGTH;

        if let Some(error) = &self.error {
            length += Self::ERROR_LENGTH;

            if let Some(message) = &error.error_message {
                length += message.len()
            }
        }

        length
    }
}

impl WritableAVP for ResultCode {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be_unchecked(self.code.into());
        if let Some(error) = &self.error {
            writer.write_u16_be_unchecked(error.error_type.into());

            if let Some(message) = &error.error_message {
                writer.write_bytes_unchecked(message.as_bytes());
            }
        }
    }
}
