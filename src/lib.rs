#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "midi1-channel-voice")]
pub mod channel_voice1;
#[cfg(feature = "midi2-channel-voice")]
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
pub mod utility;

pub mod buffer;
pub mod error;
pub mod result;

mod detail;
mod message;
mod traits;

pub use ux;

pub use message::*;
pub use traits::*;

pub mod prelude {
    pub use super::*;
    pub use crate::ux::*;
    pub use utility::JitterReduction;
}
