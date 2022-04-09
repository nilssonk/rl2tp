use crate::avp::header::{Flags, Header};
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
    fn get_length(&self) -> u16 {
        Header::LENGTH + Self::LENGTH
    }
}

impl WritableAVP for AssignedSessionId {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        let header = Header {
            flags: Flags::new(true, false),
            payload_length: Self::LENGTH,
            vendor_id: 0,
            attribute_type: Self::ATTRIBUTE_TYPE,
        };
        header.write(writer);

        writer.write_u16_be_unchecked(self.value);
    }
}
