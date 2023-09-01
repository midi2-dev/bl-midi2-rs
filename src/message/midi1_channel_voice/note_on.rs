use crate::{
    message::{
        helpers as message_helpers,
        midi1_channel_voice::{helpers as midi1cv_helpers, TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE},
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
    pub fn builder(buffer: &mut [u32]) -> NoteOnBuilder {
        NoteOnBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn velocity(&self) -> u7 {
        midi1cv_helpers::note_velocity_from_packet(self.0)
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct NoteOnBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> NoteOnBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 1) {
            Ok(()) => {
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI1_CHANNEL_VOICE_TYPE, buffer);
                Self(Ok(buffer))
            }
            Err(e) => Self(Err(e)),
        }
    }
    pub fn group(&mut self, v: u4) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(v, buffer);
        }
        self
    }
    pub fn channel(&mut self, v: u4) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(v, buffer);
        }
        self
    }
    pub fn note(&mut self, v: u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn velocity(&mut self, v: u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            midi1cv_helpers::write_note_velocity_to_packet(v, buffer);
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
    use crate::util::random_buffer;

    #[test]
    fn builder() {
        assert_eq!(
            NoteOnMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0xD))
                .channel(u4::new(0xE))
                .note(u7::new(0x75))
                .velocity(u7::new(0x3D))
                .build(),
            Ok(NoteOnMessage(&[0x2D9E_753D])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x2D9E_753D]).unwrap().group(),
            u4::new(0xD),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x2D9E_753D]).unwrap().channel(),
            u4::new(0xE),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x2D9E_753D]).unwrap().note(),
            u7::new(0x75),
        );
    }

    #[test]
    fn velocity() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x2D9E_753D]).unwrap().velocity(),
            u7::new(0x3D),
        );
    }
}
