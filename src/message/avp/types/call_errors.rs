use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

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
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete CallErrors AVP encountered");
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
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);

        // Reserved
        writer.write_bytes_unchecked(&[0x00, 0x00]);

        writer.write_u32_be_unchecked(self.crc_errors);
        writer.write_u32_be_unchecked(self.framing_errors);
        writer.write_u32_be_unchecked(self.hardware_overruns);
        writer.write_u32_be_unchecked(self.buffer_overruns);
        writer.write_u32_be_unchecked(self.timeout_errors);
        writer.write_u32_be_unchecked(self.alignment_errors);
    }
}
