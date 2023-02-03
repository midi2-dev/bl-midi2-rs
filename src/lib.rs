#![no_std]

pub mod ci;
pub mod error;
pub mod message;

mod util;

#[cfg(feature = "std")]
extern crate std;