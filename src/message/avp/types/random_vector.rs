use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
use core::borrow::Borrow;

const G_RANDOM_VECTOR_LENGTH: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RandomVector {
    pub value: [u8; G_RANDOM_VECTOR_LENGTH],
}

impl RandomVector {
    const ATTRIBUTE_TYPE: u16 = 36;
    const LENGTH: usize = G_RANDOM_VECTOR_LENGTH;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete Random Vector AVP payload encountered");
        }

        Ok(Self {
            value: reader
                .bytes(4)?
                .borrow()
                .try_into()
                .map_err(|_| "Insufficient data")?,
        })
    }
}

impl From<[u8; G_RANDOM_VECTOR_LENGTH]> for RandomVector {
    fn from(value: [u8; G_RANDOM_VECTOR_LENGTH]) -> Self {
        Self { value }
    }
}

impl From<RandomVector> for [u8; G_RANDOM_VECTOR_LENGTH] {
    fn from(value: RandomVector) -> Self {
        value.value
    }
}

impl QueryableAVP for RandomVector {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for RandomVector {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_bytes(&self.value);
    }
}
