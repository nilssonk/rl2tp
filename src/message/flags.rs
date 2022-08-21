#[cfg(test)]
mod tests;

use crate::common::{Reader, ResultStr, Writer};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MessageFlagType {
    Control,
    Data,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Flags {
    data: u16,
}

impl Flags {
    #[inline]
    pub fn new(
        message_type: MessageFlagType,
        has_length: bool,
        has_ns_nr: bool,
        has_offset: bool,
        is_prioritized: bool,
        version: u8,
    ) -> Self {
        let mut result = Self::default();
        result.set_type(message_type);
        if has_length {
            result.set_length();
        }
        if has_ns_nr {
            result.set_ns_nr();
        }
        if has_offset {
            result.set_offset();
        }
        if is_prioritized {
            result.set_prioritized();
        }
        result.set_version(version);

        result
    }

    #[inline]
    pub unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(self.data);
    }

    #[inline]
    pub fn read<T>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < 2 {
            return Err("Incomplete flag section encountered");
        }
        let data = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { data })
    }

    #[inline]
    fn get_bit(&self, i: i8) -> bool {
        (self.data >> i) & 0x1 != 0
    }

    #[inline]
    fn set_bit(&mut self, i: i8) {
        let value: u16 = 0x1 << i;
        self.data |= value;
    }

    #[inline]
    pub fn get_type(&self) -> MessageFlagType {
        if self.get_bit(8) {
            MessageFlagType::Control
        } else {
            MessageFlagType::Data
        }
    }

    #[inline]
    pub fn set_type(&mut self, message_type: MessageFlagType) {
        if let MessageFlagType::Control = message_type {
            self.set_bit(8)
        }
    }

    #[inline]
    pub fn has_length(&self) -> bool {
        self.get_bit(9)
    }

    #[inline]
    pub fn set_length(&mut self) {
        self.set_bit(9);
    }

    #[inline]
    pub fn reserved_bits_ok(&self) -> bool {
        [0, 1, 2, 3, 10, 11, 13]
            .into_iter()
            .all(|i| !self.get_bit(i))
    }

    #[inline]
    pub fn has_ns_nr(&self) -> bool {
        self.get_bit(12)
    }

    #[inline]
    pub fn set_ns_nr(&mut self) {
        self.set_bit(12);
    }

    #[inline]
    pub fn has_offset(&self) -> bool {
        self.get_bit(14)
    }

    #[inline]
    pub fn set_offset(&mut self) {
        self.set_bit(14);
    }

    #[inline]
    pub fn is_prioritized(&self) -> bool {
        self.get_bit(15)
    }

    #[inline]
    pub fn set_prioritized(&mut self) {
        self.set_bit(15);
    }

    #[inline]
    pub fn get_version(&self) -> u8 {
        ((self.data >> 4) & 0xf) as u8
    }

    #[inline]
    pub fn set_version(&mut self, version: u8) {
        assert!(
            version <= 0xf,
            "Version must be at most 0xf to not be truncated"
        );
        self.data &= 0xff0f;
        self.data |= ((version & 0xf) as u16) << 4;
    }
}
