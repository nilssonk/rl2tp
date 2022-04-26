use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

const G_LENGTH: u16 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomVector {
    pub value: [u8; G_LENGTH as usize],
}

impl RandomVector {
    const LENGTH: u16 = G_LENGTH;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete Random Vector AVP payload encountered");
        }

        Ok(Self {
            value: unsafe { reader.peek_bytes(4)?.try_into().unwrap_unchecked() },
        })
    }
}

impl QueryableAVP for RandomVector {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, 0)
    }
}

impl WritableAVP for RandomVector {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
