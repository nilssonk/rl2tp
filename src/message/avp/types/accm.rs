use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};
use core::borrow::Borrow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Accm {
    pub send_accm: [u8; 4],
    pub receive_accm: [u8; 4],
}

impl Accm {
    const ATTRIBUTE_TYPE: u16 = 35;
    const LENGTH: usize = 10;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        // Skip reserved
        reader.skip_bytes(2);

        let mut get_chunk = || {
            reader
                .bytes(4)
                .ok_or(DecodeError::AVPReadError(Self::ATTRIBUTE_TYPE))?
                .borrow()
                .try_into()
                .map_err(|_| DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE))
        };

        let send_accm = get_chunk()?;
        let receive_accm = get_chunk()?;

        Ok(Self {
            send_accm,
            receive_accm,
        })
    }
}

impl QueryableAVP for Accm {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for Accm {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);

        // Reserved
        writer.write_bytes(&[0x00, 0x00]);

        writer.write_bytes(&self.send_accm);
        writer.write_bytes(&self.receive_accm);
    }
}
