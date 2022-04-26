use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct Q931CauseCode {
    pub cause_code: u16,
    pub cause_msg: u8,
    pub advisory: Option<String>,
}

impl Q931CauseCode {
    const FIXED_LENGTH: u16 = 3;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::FIXED_LENGTH as usize {
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

impl QueryableAVP for Q931CauseCode {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        const ATTRIBUTE_TYPE: u16 = 0;

        if let Some(value) = &self.advisory {
            assert!(value.len() <= (u16::MAX - Self::FIXED_LENGTH) as usize);

            (Self::FIXED_LENGTH + value.len() as u16, ATTRIBUTE_TYPE)
        } else {
            (Self::FIXED_LENGTH, ATTRIBUTE_TYPE)
        }
    }
}

impl WritableAVP for Q931CauseCode {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
