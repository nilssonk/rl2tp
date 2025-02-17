use crate::avp::types::ResultCode;
use crate::common::{DecodeError, DecodeResult, Reader};
use core::borrow::Borrow;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u16)]
pub enum ErrorType {
    Ok,
    NoControlConnectionExists,
    WrongLength,
    OutOfRangeOrBadReserved,
    InsufficientResources,
    InvalidSessionId,
    Generic,
    TryAnotherDestination,
    UnknownMandatoryAvp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    pub error_type: ErrorType,
    pub error_message: Option<String>,
}

impl Error {
    #[inline]
    pub(crate) unsafe fn try_read<T: Borrow<[u8]>>(
        reader: &mut impl Reader<T>,
    ) -> DecodeResult<Self> {
        let error_raw = reader.read_u16_be_unchecked();
        let error_type = error_raw
            .try_into()
            .map_err(|_| DecodeError::InvalidResultCodeErrorType(error_raw))?;

        let error_message = if !reader.is_empty() {
            let data = reader
                .bytes(reader.len())
                .ok_or(DecodeError::AVPReadError(ResultCode::ATTRIBUTE_TYPE))?;
            Some(
                std::str::from_utf8(data.borrow())
                    .map_err(|_| DecodeError::InvalidUtf8(ResultCode::ATTRIBUTE_TYPE))?
                    .to_owned(),
            )
        } else {
            None
        };

        Ok(Self {
            error_type,
            error_message,
        })
    }
}
