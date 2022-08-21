use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BearerType {
    data: u32,
}

impl BearerType {
    const ATTRIBUTE_TYPE: u16 = 18;
    const LENGTH: usize = 4;

    #[inline]
    pub fn new(analog_request: bool, digital_request: bool) -> Self {
        let analog_request_bit = (analog_request as u32) << 6;
        let digital_request_bit = (digital_request as u32) << 7;
        Self {
            data: analog_request_bit | digital_request_bit,
        }
    }

    pub fn try_read<T>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete BearerType AVP encountered");
        }

        let data = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { data })
    }

    pub fn is_analog_request(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_digital_request(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}

impl QueryableAVP for BearerType {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for BearerType {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_u32_be_unchecked(self.data);
    }
}
