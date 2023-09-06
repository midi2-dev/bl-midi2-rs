use crate::{
    message::{
        helpers as message_helpers,
        midi1_channel_voice::{helpers as midi1cv_helpers, TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE},
    },
    result::Result,
    util::debug,
    *,
};

const OP_CODE: u4 = u4::new(0b1000);

#[derive(Clone, PartialEq, Eq)]
pub struct NoteOffMessage<'a>(&'a [u32]);

debug::message_debug_impl!(NoteOffMessage);

impl<'a> NoteOffMessage<'a> {
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
}

impl<'a> Message<'a> for NoteOffMessage<'a> {
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

impl<'a> Buildable<'a> for NoteOffMessage<'a> {
    type Builder = NoteOffBuilder<'a>;
}

impl<'a> GroupedMessage<'a> for NoteOffMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct NoteOffBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> NoteOffBuilder<'a> {
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
    pub fn velocity(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            midi1cv_helpers::write_note_velocity_to_packet(v, buffer);
        }
        self
    }
}

impl<'a> Builder<'a> for NoteOffBuilder<'a> {
    type Message = NoteOffMessage<'a>;
    fn build(self) -> Result<NoteOffMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(NoteOffMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 1) {
            Ok(()) => {
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI1_CHANNEL_VOICE_TYPE, buffer);
                Self(Ok(buffer))
            }
            Err(e) => Self(Err(e)),
        }
    }
}

impl<'a> GroupedBuilder<'a> for NoteOffBuilder<'a> {
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
            NoteOffMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0x1))
                .channel(u4::new(0xA))
                .note(u7::new(0x68))
                .velocity(u7::new(0x1B))
                .build(),
            Ok(NoteOffMessage(&[0x218A_681B])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x218A_681B]).unwrap().group(),
            u4::new(0x1),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x218A_681B]).unwrap().channel(),
            u4::new(0xA),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x218A_681B]).unwrap().note(),
            u7::new(0x68),
        );
    }

    #[test]
    fn velocity() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x218A_681B])
                .unwrap()
                .velocity(),
            u7::new(0x1B),
        );
    }
}
