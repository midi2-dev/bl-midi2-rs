pub mod error;
pub mod extended_system_exclusive;
pub mod message;
pub mod midi1;
pub mod midi2;
pub mod packet;
pub mod system_common;
pub mod system_exclusive;
pub mod utility;

mod bounded;
mod helpers;

// todo: remove this
use packet::Packet;
