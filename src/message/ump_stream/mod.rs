use crate::{numeric_types::u10, util::Truncate};
mod ump_stream_group;

pub mod device_identity;
pub mod endpoint_discovery;
pub mod endpoint_info;
pub mod endpoint_name;

const TYPE_CODE: u32 = 0xF;

fn status_from_buffer(buffer: &[u32]) -> u10 {
    (buffer[0] >> 16).truncate()
}
