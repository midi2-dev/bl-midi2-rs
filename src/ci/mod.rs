pub use confirm_protocol::Message as ConfirmProtocolMessage;
pub use confirm_protocol::Builder as ConfirmProtocolMessageBuilder;
pub use discovery::query::Message as DiscoveryQueryMessage;
pub use discovery::query::Builder as DiscoveryQueryMessageBuilder;
pub use discovery::reply::Message as DiscoveryReplyMessage;
pub use discovery::reply::Builder as DiscoveryReplyMessageBuilder;
pub use initiate_protocol_negotiation::query::Message as InitiateProtocolNegotiationQueryMessage;
pub use initiate_protocol_negotiation::query::Builder as InitiateProtocolNegotiationQueryMessageBuilder;
pub use initiate_protocol_negotiation::reply::Message as InitiateProtocolNegotiationReplyMessage;
pub use initiate_protocol_negotiation::reply::Builder as InitiateProtocolNegotiationReplyMessageBuilder;
pub use invalidate_muid::Message as InvalidateMuidMessage;
pub use invalidate_muid::Builder as InvalidateMuidMessageBuilder;
pub use nak::Message as NakBuilder;
pub use nak::Builder as NakMessageBuilderBuilder;
pub use protocol::Protocol;
pub use set_protocol::Message as SetProtocolMessage;
pub use set_protocol::Builder as SetProtocolMessageBuilder;
pub use test_protocol::query::Message as TestProtocolQueryMessage;
pub use test_protocol::query::Builder as TestProtocolQueryMessageBuilder;
pub use test_protocol::reply::Message as TestProtocolReplyMessage;
pub use test_protocol::reply::Builder as TestProtocolReplyMessageBuilder;

mod confirm_protocol;
mod discovery;
mod helpers;
mod initiate_protocol_negotiation;
mod invalidate_muid;
mod message_trait;
mod nak;
mod protocol;
mod set_protocol;
mod test_protocol;

pub use message_trait::CiMessage;
use message_trait::ci_message_impl;

const VERSION: u8 = 0x01;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceId {
    Channel(ux::u4),
    MidiPort,
}
