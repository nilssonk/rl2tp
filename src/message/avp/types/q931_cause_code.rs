use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
use core::borrow::Borrow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Q931CauseCode {
    pub cause_code: u16,
    pub cause_msg: u8,
    pub advisory: Option<String>,
}

impl Q931CauseCode {
    const ATTRIBUTE_TYPE: u16 = 12;
    const FIXED_LENGTH: usize = 3;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::FIXED_LENGTH as usize {
            return Err("Incomplete Q931CauseCode AVP encountered");
        }

        let cause_code = unsafe { reader.read_u16_be_unchecked() };
        let cause_msg = unsafe { reader.read_u8_unchecked() };

        let advisory = if !reader.is_empty() {
            let data = reader.bytes(reader.len())?;
            Some(
                std::str::from_utf8(data.borrow())
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

impl QueryableAVP for Q931CauseCode {
    #[inline]
    fn get_length(&self) -> usize {
        if let Some(value) = &self.advisory {
            Self::FIXED_LENGTH + value.len()
        } else {
            Self::FIXED_LENGTH
        }
    }
}

impl WritableAVP for Q931CauseCode {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be_unchecked(self.cause_code);
        writer.write_u8_unchecked(self.cause_msg);
        if let Some(value) = &self.advisory {
            writer.write_bytes_unchecked(value.as_bytes());
        }
    }
}
