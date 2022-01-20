pub struct ControlMessage {}

pub struct DataMessage {}

pub enum Message {
    Control(ControlMessage),
    Data(DataMessage),
}

enum MessageType {
    Control,
    Data,
}

struct Flags {
    data: u16,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
