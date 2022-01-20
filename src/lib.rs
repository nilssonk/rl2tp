pub struct ControlMessage {}

pub struct DataMessage {}

pub enum Message {
    Control(ControlMessage),
    Data(DataMessage),
}

enum MessageType {
    Control,
    Data,
}

struct Flags {
    data: u16,
}

impl Flags {
    pub fn new(input: &[u8]) -> Self {
        let lsb = input[0] as u16;
        let msb = (input[1] & 0xf3) as u16;
        Self {
            data: (msb << 8) & lsb,
        }
    }

    fn get_bit(&self, i: i8) -> bool {
        (self.data >> i) & 0x1 != 0
    }

    pub fn get_type(&self) -> MessageType {
        if self.get_bit(0) {
            MessageType::Control
        } else {
            MessageType::Data
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
