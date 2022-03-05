use crate::avp::QueryableAVP;

#[derive(Clone, Debug, PartialEq)]
pub struct Hidden {
    pub data: Vec<u8>,
}

impl QueryableAVP for Hidden {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
