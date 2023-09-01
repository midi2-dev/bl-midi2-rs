use crate::{
    message::{
        helpers as message_helpers,
        midi2_channel_voice::{
            attribute, helpers as midi2cv_helpers, TYPE_CODE as MIDI2CV_TYPE_CODE,
        },
    },
    result::Result,
    util::debug,
};

const OP_CODE: ux::u4 = ux::u4::new(0b1001);

#[derive(Clone, PartialEq, Eq)]
pub struct NoteOnMessage<'a>(&'a [u32]);

debug::message_debug_impl!(NoteOnMessage);

impl<'a> NoteOnMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> NoteOnBuilder {
        NoteOnBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> ux::u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> ux::u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn velocity(&self) -> u16 {
        midi2cv_helpers::note_velocity_from_packet(self.0)
    }
    pub fn attribute(&self) -> Option<attribute::Attribute> {
        attribute::from_ump(self.0)
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(data, 2)?;
        attribute::validate_ump(data)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct NoteOnBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> NoteOnBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 2) {
            Ok(()) => {
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI2CV_TYPE_CODE, buffer);
                Self(Ok(buffer))
            }
            Err(e) => Self(Err(e)),
        }
    }
    pub fn group(&mut self, v: ux::u4) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(v, buffer);
        }
        self
    }
    pub fn channel(&mut self, v: ux::u4) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(v, buffer);
        }
        self
    }
    pub fn note(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn velocity(&mut self, v: u16) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_note_velocity_to_packet(v, buffer);
        }
        self
    }
    pub fn attribute(&mut self, v: attribute::Attribute) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            attribute::write_attribute(buffer, Some(v));
        }
        self
    }
    pub fn build(&'a self) -> Result<NoteOnMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(NoteOnMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        assert_eq!(
            NoteOnMessage::builder(&mut [0x0, 0x0])
                .group(ux::u4::new(0x8))
                .channel(ux::u4::new(0x8))
                .note(ux::u7::new(0x5E))
                .velocity(0x6A14)
                .attribute(attribute::Attribute::Pitch7_9 {
                    note: ux::u7::new(0x74),
                    pitch_up: ux::u9::new(0x18A),
                })
                .build(),
            Ok(NoteOnMessage(&[0x4898_5E03, 0x6A14_E98A]))
        );
    }

    #[test]
    fn builder_no_attribute() {
        assert_eq!(
            NoteOnMessage::builder(&mut [0x0, 0x0])
                .group(ux::u4::new(0x8))
                .channel(ux::u4::new(0x8))
                .note(ux::u7::new(0x5E))
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
            ux::u4::new(0x8),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A])
                .unwrap()
                .channel(),
            ux::u4::new(0x8),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A])
                .unwrap()
                .note(),
            ux::u7::new(0x5E),
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
                note: ux::u7::new(0x74),
                pitch_up: ux::u9::new(0x18A),
            }),
        );
    }
}
