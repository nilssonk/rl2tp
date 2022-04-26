use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct Challenge {
    pub value: Vec<u8>,
}

impl Challenge {
    const ATTRIBUTE_TYPE: u16 = 11;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete Challenge AVP encountered");
        }

        Ok(Self {
            value: reader.read_bytes(reader.len())?,
        })
    }
}

impl QueryableAVP for Challenge {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        assert!(self.value.len() <= u16::MAX as usize);

        (self.value.len() as u16, Self::ATTRIBUTE_TYPE)
    }
}

impl WritableAVP for Challenge {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        writer.write_bytes_unchecked(&self.value);
    }
}
