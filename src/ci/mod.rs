pub use confirm_protocol::ConfirmProtocolBuilder;
pub use confirm_protocol::ConfirmProtocolMessage;
pub use discovery::query::DiscoveryQueryBuilder;
pub use discovery::query::DiscoveryQueryMessage;
pub use discovery::reply::DiscoveryReplyMessage;
pub use discovery::reply::DiscoveryReplyBuilder;
pub use initiate_protocol_negotiation::query::InitiateProtocolNegotiationBuilder as InitiateProtocolNegotiationQueryBuilder;
pub use initiate_protocol_negotiation::query::InitiateProtocolNegotiationMessage as InitiateProtocolNegotiationQueryMessage;
pub use initiate_protocol_negotiation::reply::InitiateProtocolNegotiationBuilder as InitiateProtocolNegotiationReplyBuilder;
pub use initiate_protocol_negotiation::reply::InitiateProtocolNegotiationMessage as InitiateProtocolNegotiationReplyMessage;
pub use invalidate_muid::InvalidateMuidBuilder;
pub use invalidate_muid::InvalidateMuidMessage;
pub use nak::NakBuilder;
pub use nak::NakMessage;
pub use protocol::Protocol;
pub use set_protocol::SetProtocolBuilder;
pub use set_protocol::SetProtocolMessage;
//pub use test_protocol::query::Message as TestProtocolQueryMessage;
//pub use test_protocol::query::Builder as TestProtocolQueryMessageBuilder;
//pub use test_protocol::reply::Message as TestProtocolReplyMessage;
//pub use test_protocol::reply::Builder as TestProtocolReplyMessageBuilder;

mod confirm_protocol;
mod discovery;
mod helpers;
mod initiate_protocol_negotiation;
mod invalidate_muid;
mod nak;
mod protocol;
mod set_protocol;
//mod test_protocol;

//pub use message_trait::CiMessage;
//use message_trait::ci_message_impl;

const VERSION: u8 = 0x01;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceId {
    Channel(ux::u4),
    MidiPort,
}
