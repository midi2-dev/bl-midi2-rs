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

pub struct ProtocolDataIterator {
    buffer: [u8; 8],
    i: usize,
}

impl core::iter::Iterator for ProtocolDataIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 5 {
            None
        } else {
            let current = self.i;
            self.i += 1;
            Some(self.buffer[current])
        }
    }
}
