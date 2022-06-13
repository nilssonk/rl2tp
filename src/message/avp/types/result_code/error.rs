use crate::common::{Reader, ResultStr};
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
    pub(crate) unsafe fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        let error_raw = reader.read_u16_be_unchecked();
        let error_type = error_raw
            .try_into()
            .map_err(|_| "Invalid ResultCode ErrorType encountered")?;

        let error_message = if !reader.is_empty() {
            let data = reader.peek_bytes(reader.len())?;
            Some(
                std::str::from_utf8(data)
                    .map_err(|_| "Invalid ResultCode error message encountered")?
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
