pub struct Message {
    status: Status,
    data: Vec<u8>,
}

enum Status {
    Begin,
    Continue,
    End,
}

enum MessageParseError {
    InvalidStatusBit(u8),
    DataOutOfRange(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = MessageParseError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        todo!()
    }
}
