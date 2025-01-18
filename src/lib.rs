#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "channel-voice1")]
pub mod channel_voice1;
#[cfg(feature = "channel-voice2")]
pub mod channel_voice2;
#[cfg(feature = "ci")]
pub mod ci;
#[cfg(feature = "flex-data")]
pub mod flex_data;
#[cfg(feature = "sysex7")]
pub mod sysex7;
#[cfg(feature = "sysex8")]
pub mod sysex8;
#[cfg(feature = "system-common")]
pub mod system_common;
#[cfg(feature = "ump-stream")]
pub mod ump_stream;
#[cfg(feature = "utility")]
pub mod utility;

pub mod buffer;
pub mod error;

mod detail;
mod message;
mod packet;
mod packets;
mod traits;

pub use ux;

pub use message::*;
pub use packets::*;
pub use traits::*;

pub mod num {
    pub use ux::*;
    pub type Fixed7_9 = fixed::FixedU16<fixed::types::extra::U9>;
    pub type Fixed7_25 = fixed::FixedU32<fixed::types::extra::U25>;
}

pub mod prelude {
    pub use super::*;
    pub use crate::ux::*;
}
