#[cfg(test)]
mod tests;

use crate::common::{Reader, ResultStr};

#[derive(Debug, PartialEq)]
pub enum MessageFlagType {
    Control,
    Data,
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    data: u16,
}

impl Flags {
    pub fn read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < 2 {
            return Err("Incomplete flag section encountered");
        }
        let data = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { data })
    }

    fn get_bit(&self, i: i8) -> bool {
        (self.data >> i) & 0x1 != 0
    }

    pub fn get_type(&self) -> MessageFlagType {
        if self.get_bit(8) {
            MessageFlagType::Control
        } else {
            MessageFlagType::Data
        }
    }

    pub fn has_length(&self) -> bool {
        self.get_bit(9)
    }

    pub fn reserved_bits_ok(&self) -> bool {
        [0, 1, 2, 3, 10, 11, 13]
            .into_iter()
            .all(|i| !self.get_bit(i))
    }

    pub fn has_ns_nr(&self) -> bool {
        self.get_bit(12)
    }

    pub fn has_offset_size(&self) -> bool {
        self.get_bit(14)
    }

    pub fn is_prioritized(&self) -> bool {
        self.get_bit(15)
    }

    pub fn get_version(&self) -> u8 {
        ((self.data >> 4) & 0xf) as u8
    }
}
