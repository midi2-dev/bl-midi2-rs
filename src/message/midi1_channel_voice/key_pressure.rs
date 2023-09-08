use crate::{
    message::{
        helpers as message_helpers,
        midi1_channel_voice::{helpers as midi1cv_helpers, TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE},
    },
    result::Result,
    *,
};

const OP_CODE: u4 = u4::new(0b1010);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyPressureMessage<'a, B: Buffer>(&'a B::Data);

impl<'a> KeyPressureMessage<'a, Ump> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn pressure(&self) -> u7 {
        midi1cv_helpers::note_velocity_from_packet(self.0)
    }
}

impl<'a> Message<'a, Ump> for KeyPressureMessage<'a, Ump> {
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Self(buffer)
    }
}

impl<'a> Buildable<'a, Ump> for KeyPressureMessage<'a, Ump> {
    type Builder = KeyPressureBuilder<'a, Ump>;
}

impl<'a> GroupedMessage<'a> for KeyPressureMessage<'a, Ump> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct KeyPressureBuilder<'a, B: Buffer>(Result<&'a mut B::Data>);

impl<'a> KeyPressureBuilder<'a, Ump> {
    pub fn group(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(v, buffer);
        }
        self
    }
    pub fn channel(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(v, buffer);
        }
        self
    }
    pub fn note(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn pressure(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            midi1cv_helpers::write_note_velocity_to_packet(v, buffer);
        }
        self
    }
}

impl<'a> Builder<'a, Ump> for KeyPressureBuilder<'a, Ump> {
    type Message = KeyPressureMessage<'a, Ump>;
    fn build(self) -> Result<Self::Message> {
        match self.0 {
            Ok(buffer) => Ok(KeyPressureMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 1) {
            Ok(()) => {
                message_helpers::clear_buffer(&mut buffer[..1]);
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI1_CHANNEL_VOICE_TYPE, buffer);
                Self(Ok(&mut buffer[..1]))
            }
            Err(e) => Self(Err(e)),
        }
    }
}

impl<'a> GroupedBuilder<'a> for KeyPressureBuilder<'a, Ump> {
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
            KeyPressureMessage::builder(&mut Ump::random_buffer::<1>())
                .group(u4::new(0xA))
                .channel(u4::new(0x3))
                .note(u7::new(0x7F))
                .pressure(u7::new(0x5C))
                .build(),
            Ok(KeyPressureMessage::<Ump>(&[0x2AA3_7F5C])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x2AA3_7F5C])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x2AA3_7F5C])
                .unwrap()
                .channel(),
            u4::new(0x3),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x2AA3_7F5C])
                .unwrap()
                .note(),
            u7::new(0x7F),
        );
    }

    #[test]
    fn pressure() {
        assert_eq!(
            KeyPressureMessage::from_data(&[0x2AA3_7F5C])
                .unwrap()
                .pressure(),
            u7::new(0x5C),
        );
    }
}
