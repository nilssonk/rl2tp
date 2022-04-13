use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeResponse {
    pub data: [u8; 16],
}

impl ChallengeResponse {
    const LENGTH: u16 = 16;

    pub fn try_read<'a>(reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete ChallengeResponse AVP encountered");
        }

        Ok(Self {
            data: unsafe { reader.peek_bytes(16)?.try_into().unwrap_unchecked() },
        })
    }
}

impl QueryableAVP for ChallengeResponse {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for ChallengeResponse {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
