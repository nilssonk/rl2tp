use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
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
    const LENGTH: u16 = 26;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
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
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, Self::ATTRIBUTE_TYPE)
    }
}

impl WritableAVP for CallErrors {
    unsafe fn write(&self, writer: &mut dyn Writer) {
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
