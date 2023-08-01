use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
use core::borrow::Borrow;

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

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::FIXED_LENGTH {
            return Err("Incomplete ResultCode AVP payload encountered");
        }

        let code_raw = unsafe { reader.read_u16_be_unchecked() };
        let code = CodeValue::from(code_raw);

        let error = if reader.len() >= Self::ERROR_LENGTH {
            Some(unsafe { Error::try_read(reader)? })
        } else {
            None
        };

        Ok(Self { code, error })
    }
}

impl QueryableAVP for ResultCode {
    #[inline]
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
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be(self.code.into());
        if let Some(error) = &self.error {
            writer.write_u16_be(error.error_type.into());

            if let Some(message) = &error.error_message {
                writer.write_bytes(message.as_bytes());
            }
        }
    }
}
