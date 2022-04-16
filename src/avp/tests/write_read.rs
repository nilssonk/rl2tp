// use crate::avp::*;
// use crate::common::{SliceReader, VecWriter};

#[allow(unused_macros)] // Remove upon first use
macro_rules! io_tests {
    [$($name:ident => $input:expr),+] => {
        $(
        #[test]
        fn $name() {
            // Serialize input
            let input = $input;
            let mut w = VecWriter::new();
            unsafe { $input.write(&mut w) };

            // Deserialize to output
            let r = Box::new(SliceReader::from(&w.data));
            let avps = AVP::try_read_greedy(r);

            // Select first and only AVP, assert successful deserialization
            let output = avps.into_iter().next().unwrap().unwrap();

            assert_eq!(input, output);
        }
    )+
    }
}
