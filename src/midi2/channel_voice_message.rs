use crate::Packet;

#[derive(
    Debug,
    PartialEq,
)]
pub struct ChannelVoiceMessage {}

#[derive(
    Debug,
    PartialEq,
)]
pub enum ChannelVoiceMessageParseError {}

impl std::convert::TryFrom<Packet> for ChannelVoiceMessage {
    type Error = ChannelVoiceMessageParseError;
    fn try_from(_: Packet) -> Result<Self, Self::Error> {
        todo!()
    }
}
