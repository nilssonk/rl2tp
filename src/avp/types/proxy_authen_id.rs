use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProxyAuthenId {
    pub value: u8,
}

impl ProxyAuthenId {
    const LENGTH: u16 = 2;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete ProxyAuthenId AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(value.into())
    }
}

impl From<u16> for ProxyAuthenId {
    fn from(value: u16) -> Self {
        let masked = (value & 0xff) as u8;
        Self { value: masked }
    }
}

impl QueryableAVP for ProxyAuthenId {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for ProxyAuthenId {
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
