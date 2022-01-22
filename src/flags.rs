#[cfg(test)]
mod tests;

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
    pub fn from_bytes(input: &[u8]) -> Result<Self, &'static str> {
        let array: [u8; 2] = match input.try_into() {
            Ok(x) => x,
            Err(_) => return Err("Incomplete flag section"),
        };
        let data = u16::from_le_bytes(array);
        Ok(Self { data })
    }

    fn get_bit(&self, i: i8) -> bool {
        (self.data >> i) & 0x1 != 0
    }

    pub fn get_type(&self) -> MessageFlagType {
        if self.get_bit(0) {
            MessageFlagType::Control
        } else {
            MessageFlagType::Data
        }
    }

    pub fn has_length(&self) -> bool {
        self.get_bit(1)
    }

    pub fn reserved_bits_ok(&self) -> bool {
        [2, 3, 5, 8, 9, 10, 11]
            .into_iter()
            .all(|i| !self.get_bit(i))
    }

    pub fn has_ns_nr(&self) -> bool {
        self.get_bit(4)
    }

    pub fn has_offset_size(&self) -> bool {
        self.get_bit(6)
    }

    pub fn is_prioritized(&self) -> bool {
        self.get_bit(7)
    }

    pub fn get_version(&self) -> u8 {
        ((self.data >> 12) & 0xf) as u8
    }
}
