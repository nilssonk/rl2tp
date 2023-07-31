use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BearerCapabilities {
    data: u32,
}

impl BearerCapabilities {
    const ATTRIBUTE_TYPE: u16 = 4;
    const LENGTH: usize = 4;

    pub fn new(digital_access_supported: bool, analog_access_supported: bool) -> Self {
        let da_bit = (digital_access_supported as u32) << 6;
        let aa_bit = (analog_access_supported as u32) << 7;

        Self {
            data: da_bit | aa_bit,
        }
    }

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete BearerCapabilities AVP encountered");
        }

        let data = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { data })
    }

    pub fn is_analog_access_supported(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_digital_access_supported(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}

impl QueryableAVP for BearerCapabilities {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for BearerCapabilities {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u32_be(self.data);
    }
}
