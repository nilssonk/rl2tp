#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Flags {
    data: u8,
}

impl Flags {
    #[inline]
    pub fn from(input: u8) -> Self {
        Self { data: input & 0x3f }
    }

    #[inline]
    fn get_bit(&self, i: i8) -> bool {
        (self.data >> i) & 0x1 != 0
    }

    #[allow(dead_code)] // Remove upon first use
    #[inline]
    pub fn is_mandatory(&self) -> bool {
        self.get_bit(0)
    }

    #[inline]
    pub fn is_hidden(&self) -> bool {
        self.get_bit(1)
    }

    #[allow(dead_code)] // Remove upon first use
    #[inline]
    pub fn reserved_bits_ok(&self) -> bool {
        (2..6).all(|i| !self.get_bit(i))
    }
}
