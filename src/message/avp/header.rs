mod flags;
pub(crate) use flags::Flags;

use crate::common::Reader;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Header {
    pub flags: Flags,
    pub payload_length: u16,
    pub vendor_id: u16,
    pub attribute_type: u16,
}

impl Header {
    pub const LENGTH: u16 = 6;

    #[inline]
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> Option<Self> {
        // Note: Subsequent unsafe code depends on this check
        if reader.len() < Self::LENGTH as usize {
            return None;
        }

        // Flags and length share the first 2 octets
        let octet1 = unsafe { reader.read_u8_unchecked() };
        let octet2 = unsafe { reader.read_u8_unchecked() };

        let flags = Flags::from(octet1);

        let msb = (octet1 >> 6) as u16;
        let lsb = octet2 as u16;
        let length = msb << 8 | lsb;

        // The second 2 octets are the Vendor ID
        let vendor_id = unsafe { reader.read_u16_be_unchecked() };

        // The final 2 octets are the Attribute Type
        let attribute_type = unsafe { reader.read_u16_be_unchecked() };

        let payload_length = length - Self::LENGTH;

        Some(Header {
            flags,
            payload_length,
            vendor_id,
            attribute_type,
        })
    }
}
