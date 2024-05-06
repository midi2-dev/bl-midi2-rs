#![no_std]
// #![doc = include_str!("../README.md")]

#[cfg(any(feature = "std", test))]
extern crate std;

pub mod buffer;
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

pub mod error;
pub mod result;

mod detail;
mod traits;

pub use ux;

pub use traits::*;

pub mod numeric_types {
    #[rustfmt::skip]
    pub use ux::{
        u1, u2, u3, u4, u5, u6, u7, u9, u10, u11, u12,
        u13, u14, u15, u17, u18, u19, u20, u21, u22,
        u23, u24, u25, u26, u27, u28, u29, u30, u31,
    };
}

pub mod prelude {
    #[cfg(feature = "midi1-channel-voice")]
    pub use crate::channel_voice1;
    #[cfg(feature = "midi2-channel-voice")]
    pub use crate::channel_voice2;
    #[cfg(feature = "sysex7")]
    pub use crate::sysex7;
    #[cfg(feature = "system-common")]
    pub use crate::system_common;
    pub use crate::{numeric_types::*, traits::*};
}
