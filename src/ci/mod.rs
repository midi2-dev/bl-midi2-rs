pub use discovery::query::DiscoveryQueryBuilder;
pub use discovery::query::DiscoveryQueryMessage;
pub use discovery::reply::DiscoveryReplyBuilder;
pub use discovery::reply::DiscoveryReplyMessage;
pub use invalidate_muid::InvalidateMuidBuilder;
pub use invalidate_muid::InvalidateMuidMessage;
pub use nak::NakBuilder;
pub use nak::NakMessage;

mod discovery;
mod helpers;
mod invalidate_muid;
mod nak;

const VERSION: u8 = 0x01;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceId {
    Channel(ux::u4),
    MidiPort,
}
