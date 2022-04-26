use crate::avp::Header;
use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareRevision {
    pub value: u16,
}

impl FirmwareRevision {
    const ATTRIBUTE_TYPE: u16 = 6;
    const LENGTH: u16 = 2;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete FirmwareRevision AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u16> for FirmwareRevision {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl QueryableAVP for FirmwareRevision {
    fn get_length(&self) -> u16 {
        Header::LENGTH + Self::LENGTH
    }
}

impl WritableAVP for FirmwareRevision {
    unsafe fn write(&self, writer: &mut impl Writer) {
        let header =
            Header::with_payload_length_and_attribute_type(Self::LENGTH, Self::ATTRIBUTE_TYPE);
        header.write(writer);

        writer.write_u16_be_unchecked(self.value);
    }
}
