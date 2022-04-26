use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

const CHALLENGE_RESPONSE_LENGTH: u16 = 16;

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeResponse {
    pub value: [u8; CHALLENGE_RESPONSE_LENGTH as usize],
}

impl ChallengeResponse {
    const ATTRIBUTE_TYPE: u16 = 13;
    const LENGTH: u16 = CHALLENGE_RESPONSE_LENGTH;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete ChallengeResponse AVP encountered");
        }

        Ok(Self {
            value: unsafe { reader.peek_bytes(16)?.try_into().unwrap_unchecked() },
        })
    }
}

impl QueryableAVP for ChallengeResponse {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, Self::ATTRIBUTE_TYPE)
    }
}

impl WritableAVP for ChallengeResponse {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        writer.write_bytes_unchecked(&self.value);
    }
}
