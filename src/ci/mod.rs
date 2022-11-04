use crate::{
    message::system_exclusive_8bit::Message as Sysex8Message,
    error::Error,
};

pub mod discovery;
mod helpers;

const VERSION: u8 = 0x01;

pub trait CiMessage : Sized {
    fn to_sysex_8<'a>(&self, messages: &'a mut [Sysex8Message], stream_id: u8) -> &'a [Sysex8Message];
    fn from_sysex_8(messages: &[Sysex8Message]) -> Self;
    fn validate_sysex_8(message: &[Sysex8Message]) -> Result<(), Error>;
    fn try_from_sysex_8(messages: &[Sysex8Message]) -> Result<Self, Error> {
        <Self as CiMessage>::validate_sysex_8(messages)?;
        Ok(<Self as CiMessage>::from_sysex_8(messages))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceId {
    Channel(ux::u4),
    MidiPort,
}
