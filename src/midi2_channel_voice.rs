use crate::Packet;

#[derive(
    Debug,
    PartialEq,
)]
pub struct Message {}

#[derive(
    Debug,
    PartialEq,
)]
pub enum MessageParseError {}

impl std::convert::TryFrom<Packet> for Message {
    type Error = MessageParseError;
    fn try_from(_: Packet) -> Result<Self, Self::Error> {
        todo!()
    }
}
