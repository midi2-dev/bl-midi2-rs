use crate::error::Error;

mod helpers;
pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod system_common;
pub mod system_exclusive_7bit;
pub mod system_exclusive_8bit;
pub mod utility;

pub trait Midi2Message: Sized {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error>;
    fn from_ump(bytes: &[u32]) -> Self;
    fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32];
    fn try_from_ump(bytes: &[u32]) -> Result<Self, Error> {
        <Self as Midi2Message>::validate_ump(bytes)?;
        Ok(<Self as Midi2Message>::from_ump(bytes))
    }
}
