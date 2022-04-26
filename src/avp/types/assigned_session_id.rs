use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignedSessionId {
    pub value: u16,
}

impl AssignedSessionId {
    const ATTRIBUTE_TYPE: u16 = 14;
    const LENGTH: u16 = 2;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
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
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, Self::ATTRIBUTE_TYPE)
    }
}

impl WritableAVP for AssignedSessionId {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        writer.write_u16_be_unchecked(self.value);
    }
}
