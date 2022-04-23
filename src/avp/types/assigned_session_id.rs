use crate::avp::header::Header;
use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignedSessionId {
    pub value: u16,
}

impl AssignedSessionId {
    const ATTRIBUTE_TYPE: u16 = 14;
    const LENGTH: u16 = 2;

    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete AssignedSessionId AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u16> for AssignedSessionId {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl QueryableAVP for AssignedSessionId {
    fn get_length(&self) -> u16 {
        Header::LENGTH + Self::LENGTH
    }
}

impl WritableAVP for AssignedSessionId {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        let header =
            Header::with_payload_length_and_attribute_type(Self::LENGTH, Self::ATTRIBUTE_TYPE);
        header.write(writer);

        writer.write_u16_be_unchecked(self.value);
    }
}
