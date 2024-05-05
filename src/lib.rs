#![no_std]
// #![doc = include_str!("../README.md")]

#[cfg(any(feature = "std", test))]
extern crate std;

pub mod buffer;
#[cfg(feature = "ci")]
pub mod ci;
pub mod message;

#[cfg(test)]
pub(crate) mod test_support;

mod error;
mod result;
mod traits;
mod util;

pub use error::*;
pub use numeric_types::*;
pub use result::*;
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
    #[cfg(feature = "sysex7")]
    pub use crate::message::sysex7;
    #[cfg(feature = "system-common")]
    pub use crate::message::system_common;
    pub use crate::{numeric_types::*, traits::*};
}
