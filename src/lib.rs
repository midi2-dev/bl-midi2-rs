#![no_std]
#![cfg_attr(feature = "std", doc = include_str!("../README.md"))]

#[cfg(feature = "std")]
extern crate std;

pub mod ci;
pub mod message;

mod buffer;
mod error;
mod result;
mod traits;
mod util;

use buffer::*;
pub use error::*;
pub use numeric_types::*;
pub use result::*;
pub use traits::*;

pub use message::MessageBorrowed;
pub use message::MessageOwned;

pub mod numeric_types {
    #[rustfmt::skip]
    pub use ux::{
        u1, u2, u3, u4, u5, u6, u7, u9, u10, u11, u12,
        u13, u14, u15, u17, u18, u19, u20, u21, u22,
        u23, u24, u25, u26, u27, u28, u29, u30, u31,
    };
}

pub mod prelude {
    pub use crate::{message::Message, numeric_types::*, traits::*};
}
