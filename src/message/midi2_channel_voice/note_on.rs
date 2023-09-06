use crate::{
    message::{
        helpers as message_helpers,
        midi2_channel_voice::{
            attribute, helpers as midi2cv_helpers, TYPE_CODE as MIDI2CV_TYPE_CODE,
        },
    },
    result::Result,
    util::debug,
    *,
};

const OP_CODE: u4 = u4::new(0b1001);

#[derive(Clone, PartialEq, Eq)]
pub struct NoteOnMessage<'a>(&'a [u32]);

debug::message_debug_impl!(NoteOnMessage);

impl<'a> NoteOnMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn velocity(&self) -> u16 {
        midi2cv_helpers::note_velocity_from_packet(self.0)
    }
    pub fn attribute(&self) -> Option<attribute::Attribute> {
        attribute::from_ump(self.0)
    }
}

impl<'a> Message<'a> for NoteOnMessage<'a> {
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(buffer, 2)?;
        attribute::validate_ump(buffer)?;
        Ok(())
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Self(buffer)
    }
}

impl<'a> Buildable<'a> for NoteOnMessage<'a> {
    type Builder = NoteOnBuilder<'a>;
}

impl<'a> GroupedMessage<'a> for NoteOnMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct NoteOnBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> NoteOnBuilder<'a> {
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
    pub fn velocity(mut self, v: u16) -> Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_note_velocity_to_packet(v, buffer);
        }
        self
    }
    pub fn attribute(mut self, v: attribute::Attribute) -> Self {
        if let Ok(buffer) = &mut self.0 {
            attribute::write_attribute(buffer, Some(v));
        }
        self
    }
}

impl<'a> Builder<'a> for NoteOnBuilder<'a> {
    type Message = NoteOnMessage<'a>;
    fn build(self) -> Result<NoteOnMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(NoteOnMessage(buffer)),
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

impl<'a> GroupedBuilder<'a> for NoteOnBuilder<'a> {
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
    use crate::util::random_buffer;

    #[test]
    fn builder() {
        assert_eq!(
            NoteOnMessage::builder(&mut random_buffer::<2>())
                .group(u4::new(0x8))
                .channel(u4::new(0x8))
                .note(u7::new(0x5E))
                .velocity(0x6A14)
                .attribute(attribute::Attribute::Pitch7_9 {
                    note: u7::new(0x74),
                    pitch_up: u9::new(0x18A),
                })
                .build(),
            Ok(NoteOnMessage(&[0x4898_5E03, 0x6A14_E98A]))
        );
    }

    #[test]
    fn builder_no_attribute() {
        assert_eq!(
            NoteOnMessage::builder(&mut random_buffer::<2>())
                .group(u4::new(0x8))
                .channel(u4::new(0x8))
                .note(u7::new(0x5E))
                .velocity(0x6A14)
                .build(),
            Ok(NoteOnMessage(&[0x4898_5E00, 0x6A14_0000]))
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A])
                .unwrap()
                .group(),
            u4::new(0x8),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A])
                .unwrap()
                .channel(),
            u4::new(0x8),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A])
                .unwrap()
                .note(),
            u7::new(0x5E),
        );
    }

    #[test]
    fn volocity() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A])
                .unwrap()
                .velocity(),
            0x6A14,
        );
    }

    #[test]
    fn attribute() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A])
                .unwrap()
                .attribute(),
            Some(attribute::Attribute::Pitch7_9 {
                note: u7::new(0x74),
                pitch_up: u9::new(0x18A),
            }),
        );
    }
}
