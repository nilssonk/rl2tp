#[cfg(test)]
mod tests;

use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliceReader<'a> {
    data: &'a [u8],
}

impl<'a> SliceReader<'a> {
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

impl<'a> Reader<'a> for SliceReader<'a> {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn subreader(&mut self, length: usize) -> Box<dyn Reader<'a> + 'a> {
        let new_reader = Box::new(SliceReader::from(&self.data[..length]));
        self.data = &self.data[length..];
        new_reader
    }

    fn skip_bytes(&mut self, length: usize) {
        self.data = &self.data[length..];
    }

    fn peek_bytes(&self, length: usize) -> ResultStr<&'a [u8]> {
        self.data.get(..length).ok_or("Incomplete data")
    }

    fn read_bytes(&mut self, length: usize) -> ResultStr<Vec<u8>> {
        let result = self
            .data
            .get(..length)
            .map(|x| x.to_owned())
            .ok_or("Incomplete data");
        self.data = &self.data[length..];
        result
    }

    unsafe fn read_u8_unchecked(&mut self) -> u8 {
        let result = self.data.get_unchecked(0);
        self.data = &self.data[1..];
        *result
    }

    unsafe fn read_u16_be_unchecked(&mut self) -> u16 {
        let buf = read_buf_unchecked!(self.data, 2);
        u16::from_be_bytes(buf)
    }

    unsafe fn read_u32_be_unchecked(&mut self) -> u32 {
        let buf = read_buf_unchecked!(self.data, 4);
        u32::from_be_bytes(buf)
    }

    unsafe fn read_u64_be_unchecked(&mut self) -> u64 {
        let buf = read_buf_unchecked!(self.data, 8);
        u64::from_be_bytes(buf)
    }
}
