use crate::{
    message::helpers as message_helpers,
    message::midi2_channel_voice::TYPE_CODE as MIDI2CV_TYPE_CODE, result::Result, util::debug, *,
};

#[derive(Clone, PartialEq, Eq)]
pub struct KeyPressureMessage<'a>(&'a [u32]);

debug::message_debug_impl!(KeyPressureMessage);

const OP_CODE: u4 = u4::new(0b1010);

impl<'a> KeyPressureMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn key_pressure_data(&self) -> u32 {
        self.0[1]
    }
}

impl<'a> Message<'a, Ump> for KeyPressureMessage<'a> {
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(buffer, 2)?;
        Ok(())
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Self(buffer)
    }
}

impl<'a> Buildable<'a, Ump> for KeyPressureMessage<'a> {
    type Builder = KeyPressureBuilder<'a>;
}

impl<'a> GroupedMessage<'a> for KeyPressureMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

pub struct KeyPressureBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> KeyPressureBuilder<'a> {
    pub fn channel(mut self, channel: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(channel, buffer);
        }
        self
    }
    pub fn note(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn key_pressure_data(mut self, data: u32) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = data;
        }
        self
    }
}

impl<'a> Builder<'a, Ump> for KeyPressureBuilder<'a> {
    type Message = KeyPressureMessage<'a>;
    fn build(self) -> Result<KeyPressureMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(KeyPressureMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 2) {
            Ok(()) => {
                message_helpers::clear_buffer(&mut buffer[..2]);
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI2CV_TYPE_CODE, buffer);
                Self(Ok(&mut buffer[..2]))
            }
            Err(e) => Self(Err(e)),
        }
    }
}

impl<'a> GroupedBuilder<'a> for KeyPressureBuilder<'a> {
    fn group(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(v, buffer);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            KeyPressureMessage::builder(&mut Ump::random_buffer::<2>())
                .group(u4::new(0xB))
                .channel(u4::new(0xC))
                .note(u7::new(0x59))
                .key_pressure_data(0xC0B83064)
                .build(),
            Ok(KeyPressureMessage(&[0x4BAC_5900, 0xC0B83064])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x4BAC_5900, 0xC0B83064])
                .unwrap()
                .group(),
            u4::new(0xB),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x4BAC_5900, 0xC0B83064])
                .unwrap()
                .channel(),
            u4::new(0xC),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x4BAC_5900, 0xC0B83064])
                .unwrap()
                .note(),
            u7::new(0x59),
        );
    }

    #[test]
    fn key_pressure_data() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x4BAC_5900, 0xC0B83064])
                .unwrap()
                .key_pressure_data(),
            0xC0B83064,
        );
    }
}
