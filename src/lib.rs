pub mod message;
pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod packet;
pub mod system;

pub use packet::Packet;

type Channel = u8;
type Controller = u8;
type Group = u8;
type Note = u8;
type Program = u8;
type Value = u8;
type Velocity = u8;
