#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Flags {
    data: u8,
}

impl Flags {
    pub fn new(is_mandatory: bool, is_hidden: bool) -> Self {
        Self {
            data: is_mandatory as u8 | (is_hidden as u8) << 1,
        }
    }

    pub fn from(input: u8) -> Self {
        Self { data: input & 0x3f }
    }

    fn get_bit(&self, i: i8) -> bool {
        (self.data >> i) & 0x1 != 0
    }

    pub fn is_mandatory(&self) -> bool {
        self.get_bit(0)
    }

    pub fn is_hidden(&self) -> bool {
        self.get_bit(1)
    }

    #[allow(dead_code)]
    pub fn reserved_bits_ok(&self) -> bool {
        (2..6).into_iter().all(|i| !self.get_bit(i))
    }
}
