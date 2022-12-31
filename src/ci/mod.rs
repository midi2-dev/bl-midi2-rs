pub mod discovery;
pub mod initiate_protocol_negotiation;
pub mod invalidate_muid;
pub mod nak;

mod helpers;
mod message_trait;

pub use message_trait::CiMessage;
use message_trait::CiMessageDetail;

const VERSION: u8 = 0x01;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceId {
    Channel(ux::u4),
    MidiPort,
}
