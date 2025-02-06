#![doc = include_str!("ci/README.md")]

mod common_properties;
mod device_id;
mod discovery;
mod version;

pub use device_id::*;
pub use discovery::*;
pub use version::*;

pub trait Ci<B: crate::buffer::Bytes> {
    fn device_id(&self) -> device_id::DeviceId
    where
        Self: version::CiVersion<0x1>;
    fn source(&self) -> ux::u28
    where
        Self: version::CiVersion<0x1>;
    fn destination(&self) -> ux::u28
    where
        Self: version::CiVersion<0x1>;
}
