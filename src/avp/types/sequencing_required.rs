use crate::avp::QueryableAVP;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SequencingRequired {}

impl QueryableAVP for SequencingRequired {
    fn get_length(&self) -> u16 {
        0
    }
}
