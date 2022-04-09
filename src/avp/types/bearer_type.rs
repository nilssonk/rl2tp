use crate::avp::header::{Flags, Header};
use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BearerType {
    data: u32,
}

impl BearerType {
    const ATTRIBUTE_TYPE: u16 = 18;
    const LENGTH: u16 = 4;

    pub fn new(analog_request: bool, digital_request: bool) -> Self {
        let analog_request_bit = (analog_request as u32) << 6;
        let digital_request_bit = (digital_request as u32) << 7;
        Self {
            data: analog_request_bit | digital_request_bit,
        }
    }

    pub fn from_raw(data: u32) -> Self {
        Self { data }
    }

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete BearerType AVP encountered");
        }

        let data = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self::from_raw(data))
    }

    pub fn is_analog_request(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_digital_request(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}

impl QueryableAVP for BearerType {
    fn get_length(&self) -> u16 {
        Header::LENGTH + Self::LENGTH
    }
}

impl WritableAVP for BearerType {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        let header = Header {
            flags: Flags::new(true, false),
            payload_length: Self::LENGTH,
            vendor_id: 0,
            attribute_type: Self::ATTRIBUTE_TYPE,
        };
        header.write(writer);

        writer.write_u32_be_unchecked(self.data);
    }
}
