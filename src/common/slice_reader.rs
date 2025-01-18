#[cfg(test)]
mod tests;

use crate::common::Reader;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SliceReader<'a> {
    data: &'a [u8],
}

impl<'a> SliceReader<'a> {
    #[inline]
    pub fn from(data: &'a [u8]) -> Self {
        Self { data }
    }
}

macro_rules! read_buf_unchecked {
    ( $data:expr, $n:expr ) => {{
        let result = $data.get_unchecked(..$n);
        $data = &$data[$n..];
        result.try_into().unwrap_unchecked()
    }};
}

impl<'a> Reader<&'a [u8]> for SliceReader<'a> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn subreader(&mut self, length: usize) -> Self {
        let new_reader = SliceReader::from(&self.data[..length]);
        self.data = &self.data[length..];
        new_reader
    }

    #[inline]
    fn skip_bytes(&mut self, length: usize) {
        self.data = &self.data[length..];
    }

    #[inline]
    fn bytes(&mut self, length: usize) -> Option<&'a [u8]> {
        let result = self.data.get(..length);
        self.data = &self.data[length..];
        result
    }

    #[inline]
    unsafe fn read_u8_unchecked(&mut self) -> u8 {
        let result = self.data.get_unchecked(0);
        self.data = &self.data[1..];
        *result
    }

    #[inline]
    unsafe fn read_u16_be_unchecked(&mut self) -> u16 {
        let buf = read_buf_unchecked!(self.data, 2);
        u16::from_be_bytes(buf)
    }

    #[inline]
    unsafe fn read_u32_be_unchecked(&mut self) -> u32 {
        let buf = read_buf_unchecked!(self.data, 4);
        u32::from_be_bytes(buf)
    }

    #[inline]
    unsafe fn read_u64_be_unchecked(&mut self) -> u64 {
        let buf = read_buf_unchecked!(self.data, 8);
        u64::from_be_bytes(buf)
    }
}
