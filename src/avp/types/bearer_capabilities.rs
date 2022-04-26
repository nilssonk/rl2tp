use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BearerCapabilities {
    data: u32,
}

impl BearerCapabilities {
    const ATTRIBUTE_TYPE: u16 = 4;
    const LENGTH: u16 = 4;

    pub fn new(digital_access_supported: bool, analog_access_supported: bool) -> Self {
        let da_bit = (digital_access_supported as u32) << 6;
        let aa_bit = (analog_access_supported as u32) << 7;

        Self::from_raw(da_bit | aa_bit)
    }

    pub fn from_raw(data: u32) -> Self {
        Self { data }
    }

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
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
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, Self::ATTRIBUTE_TYPE)
    }
}

impl WritableAVP for BearerCapabilities {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        writer.write_u32_be_unchecked(self.data);
    }
}
