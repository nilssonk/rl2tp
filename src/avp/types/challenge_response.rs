use crate::avp::header::Header;
use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

const CHALLENGE_RESPONSE_LENGTH: u16 = 16;

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeResponse {
    pub data: [u8; CHALLENGE_RESPONSE_LENGTH as usize],
}

impl ChallengeResponse {
    const ATTRIBUTE_TYPE: u16 = 13;
    const LENGTH: u16 = CHALLENGE_RESPONSE_LENGTH;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
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
        Header::LENGTH + Self::LENGTH
    }
}

impl WritableAVP for ChallengeResponse {
    unsafe fn write(&self, writer: &mut impl Writer) {
        let header =
            Header::with_payload_length_and_attribute_type(Self::LENGTH, Self::ATTRIBUTE_TYPE);
        header.write(writer);

        writer.write_bytes_unchecked(&self.data);
    }
}
