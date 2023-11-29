use crate::{ci::DeviceId, error::Error, result::Result};

pub fn validate_ci_standard_bytes(buffer: &[u8]) -> Result<()> {
    if buffer.len() < 14 {
        return Err(Error::BufferOverflow);
    }

    const UNIVERSAL_SYSEX_ID: u8 = 0x7E;
    if buffer[1] != UNIVERSAL_SYSEX_ID {
        return Err(Error::InvalidData);
    };

    DeviceId::from_u8(buffer[2])?;

    const MIDI_CI_SYSEX_ID: u8 = 0x0D;
    if buffer[3] != MIDI_CI_SYSEX_ID {
        return Err(Error::InvalidData);
    };

    Ok(())
}
