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
pub enum DeserializeError {}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(_: Packet) -> Result<Self, Self::Error> {
        todo!()
    }
}
