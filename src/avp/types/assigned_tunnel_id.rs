use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignedTunnelId {
    pub value: u16,
}

impl AssignedTunnelId {
    const LENGTH: u16 = 2;

    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete AssignedTunnelId AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u16> for AssignedTunnelId {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl QueryableAVP for AssignedTunnelId {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for AssignedTunnelId {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
