use super::{Flags, MessageFlagType};
use crate::common::SliceReader;

#[test]
fn from_bytes() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff]));
        assert_eq!(f, Ok(Flags { data: 0xffff }));
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00]));
        assert_eq!(f, Ok(Flags { data: 0x0000 }));
    }
    // First half
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0x00]));
        assert_eq!(f, Ok(Flags { data: 0xff00 }));
    }
    // Second half
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0xff]));
        assert_eq!(f, Ok(Flags { data: 0x00ff }));
    }

    // Incomplete
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00]));
        assert_eq!(f, Err("Incomplete flag section encountered"));
    }
}

#[test]
fn get_bit() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff])).unwrap();
        for i in 0..16 {
            let value = f.get_bit(i);
            assert_eq!(value, true);
        }
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00])).unwrap();
        for i in 0..16 {
            let value = f.get_bit(i);
            assert_eq!(value, false);
        }
    }
    // First bit in each half
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x01, 0x01])).unwrap();
        for i in 0..16 {
            let value = f.get_bit(i);
            if i == 0 || i == 8 {
                assert_eq!(value, true);
            } else {
                assert_eq!(value, false);
            }
        }
    }
    // Last bit in each half
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x80, 0x80])).unwrap();
        for i in 0..16 {
            let value = f.get_bit(i);
            if i == 7 || i == 15 {
                assert_eq!(value, true);
            } else {
                assert_eq!(value, false);
            }
        }
    }
}

#[test]
fn get_type() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff])).unwrap();
        assert_eq!(f.get_type(), MessageFlagType::Control);
    }
    // All but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xfe, 0xff])).unwrap();
        assert_eq!(f.get_type(), MessageFlagType::Data);
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00])).unwrap();
        assert_eq!(f.get_type(), MessageFlagType::Data);
    }
    // None but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x01, 0x00])).unwrap();
        assert_eq!(f.get_type(), MessageFlagType::Control);
    }
}

#[test]
fn has_length() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff])).unwrap();
        assert_eq!(f.has_length(), true);
    }
    // All but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xfd, 0xff])).unwrap();
        assert_eq!(f.has_length(), false);
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00])).unwrap();
        assert_eq!(f.has_length(), false);
    }
    // None but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x02, 0x00])).unwrap();
        assert_eq!(f.has_length(), true);
    }
}

#[test]
fn reserved_bits_ok() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff])).unwrap();
        assert_eq!(f.reserved_bits_ok(), false);
    }
    // All but relevant bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xd3, 0xf0])).unwrap();
        assert_eq!(f.reserved_bits_ok(), true);
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00])).unwrap();
        assert_eq!(f.reserved_bits_ok(), true);
    }
    // None but relevant bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x2c, 0x0f])).unwrap();
        assert_eq!(f.reserved_bits_ok(), false);
    }
}

#[test]
fn has_ns_nr() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff])).unwrap();
        assert_eq!(f.has_ns_nr(), true);
    }
    // All but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xef, 0xff])).unwrap();
        assert_eq!(f.has_ns_nr(), false);
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00])).unwrap();
        assert_eq!(f.has_ns_nr(), false);
    }
    // None but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x10, 0x00])).unwrap();
        assert_eq!(f.has_ns_nr(), true);
    }
}

#[test]
fn has_offset() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff])).unwrap();
        assert_eq!(f.has_offset(), true);
    }
    // All but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xbf, 0xff])).unwrap();
        assert_eq!(f.has_offset(), false);
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00])).unwrap();
        assert_eq!(f.has_offset(), false);
    }
    // None but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x40, 0x00])).unwrap();
        assert_eq!(f.has_offset(), true);
    }
}

#[test]
fn is_prioritized() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff])).unwrap();
        assert_eq!(f.is_prioritized(), true);
    }
    // All but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x7f, 0xff])).unwrap();
        assert_eq!(f.is_prioritized(), false);
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00])).unwrap();
        assert_eq!(f.is_prioritized(), false);
    }
    // None but relevant bit
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x80, 0x00])).unwrap();
        assert_eq!(f.is_prioritized(), true);
    }
}

#[test]
fn get_version() {
    // All bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0xff])).unwrap();
        assert_eq!(f.get_version(), 0xf);
    }
    // No bits
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0x00])).unwrap();
        assert_eq!(f.get_version(), 0x0);
    }
    // First half
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0xff, 0x00])).unwrap();
        assert_eq!(f.get_version(), 0x0);
    }
    // Second half
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0xff])).unwrap();
        assert_eq!(f.get_version(), 0xf);
    }
    // Only version
    {
        let f = Flags::read(&mut SliceReader::from(&vec![0x00, 0xf0])).unwrap();
        assert_eq!(f.get_version(), 0xf);
    }
}
