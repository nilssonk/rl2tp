use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BearerCapabilities {
    data: u32,
}

impl BearerCapabilities {
    pub fn from_raw(data: u32) -> Self {
        Self { data }
    }

    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 4 {
            return Err("Incomplete BearerCapabilities AVP encountered");
        }

        let data = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self::from_raw(data))
    }

    pub fn is_analog_access_supported(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_digital_access_supported(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}

impl QueryableAVP for BearerCapabilities {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
