use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallErrors {
    pub crc_errors: u32,
    pub framing_errors: u32,
    pub hardware_overruns: u32,
    pub buffer_overruns: u32,
    pub timeout_errors: u32,
    pub alignment_errors: u32,
}

impl CallErrors {
    const ATTRIBUTE_TYPE: u16 = 34;
    const LENGTH: usize = 26;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        // Skip reserved
        reader.skip_bytes(2);

        let crc_errors = unsafe { reader.read_u32_be_unchecked() };
        let framing_errors = unsafe { reader.read_u32_be_unchecked() };
        let hardware_overruns = unsafe { reader.read_u32_be_unchecked() };
        let buffer_overruns = unsafe { reader.read_u32_be_unchecked() };
        let timeout_errors = unsafe { reader.read_u32_be_unchecked() };
        let alignment_errors = unsafe { reader.read_u32_be_unchecked() };

        Ok(Self {
            crc_errors,
            framing_errors,
            hardware_overruns,
            buffer_overruns,
            timeout_errors,
            alignment_errors,
        })
    }
}

impl QueryableAVP for CallErrors {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for CallErrors {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);

        // Reserved
        writer.write_bytes(&[0x00, 0x00]);

        writer.write_u32_be(self.crc_errors);
        writer.write_u32_be(self.framing_errors);
        writer.write_u32_be(self.hardware_overruns);
        writer.write_u32_be(self.buffer_overruns);
        writer.write_u32_be(self.timeout_errors);
        writer.write_u32_be(self.alignment_errors);
    }
}
