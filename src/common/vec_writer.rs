#[cfg(test)]
mod tests;

use crate::common::Writer;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VecWriter {
    pub data: Vec<u8>,
}

impl VecWriter {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Writer for VecWriter {
    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    unsafe fn write_bytes_unchecked(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    #[inline]
    unsafe fn write_bytes_unchecked_at(&mut self, bytes: &[u8], offset: usize) {
        assert!(offset + bytes.len() <= self.data.len());
        std::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            self.data[offset..].as_mut_ptr(),
            bytes.len(),
        );
    }

    #[inline]
    unsafe fn write_u8_unchecked(&mut self, value: u8) {
        self.data.push(value);
    }

    #[inline]
    unsafe fn write_u16_be_unchecked(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.data.extend_from_slice(&bytes);
    }

    #[inline]
    unsafe fn write_u32_be_unchecked(&mut self, value: u32) {
        let bytes = value.to_be_bytes();
        self.data.extend_from_slice(&bytes);
    }

    #[inline]
    unsafe fn write_u64_be_unchecked(&mut self, value: u64) {
        let bytes = value.to_be_bytes();
        self.data.extend_from_slice(&bytes);
    }
}
