use crate::{
    message::{
        helpers as message_helpers,
        midi2_channel_voice::{
            TYPE_CODE as MIDI2CV_TYPE_CODE,
            helpers as midi2cv_helpers,
        },
    },
    result::Result,
    util::{BitOps, debug},
};

const OP_CODE: ux::u4 = ux::u4::new(0b0001);

#[derive(Clone, PartialEq, Eq)]
pub struct AssignablePerNoteControllerMessage<'a>(&'a [u32]);

debug::message_debug_impl!(AssignablePerNoteControllerMessage);

impl<'a> AssignablePerNoteControllerMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> AssignablePerNoteControllerBuilder {
        AssignablePerNoteControllerBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> ux::u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> ux::u7 {
        midi2cv_helpers::note_from_packet(self.0)
    }
    pub fn index(&self) -> u8 {
        self.0[0].octet(3)
    }
    pub fn controller_data(&self) -> u32 {
        self.0[1]
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        midi2cv_helpers::validate_buffer_size(data, 2)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct AssignablePerNoteControllerBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> AssignablePerNoteControllerBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        match midi2cv_helpers::validate_buffer_size(buffer, 2) {
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
            midi2cv_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn index(&mut self, v: u8) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(3, v);
        }
        self
    }
    pub fn controller_data(&mut self, v: u32) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = v;
        }
        self
    }
    pub fn build(&'a self) -> Result<AssignablePerNoteControllerMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(AssignablePerNoteControllerMessage(buffer)),
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
            AssignablePerNoteControllerMessage::builder(&mut [0x0, 0x0])
                .group(ux::u4::new(0x2))
                .channel(ux::u4::new(0x4))
                .note(ux::u7::new(0x6F))
                .index(0xB1)
                .controller_data(0x46105EE5)
                .build(),
            Ok(AssignablePerNoteControllerMessage(&[0x4214_6FB1, 0x46105EE5])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5]).unwrap().group(),
            ux::u4::new(0x2),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5]).unwrap().channel(),
            ux::u4::new(0x4),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5]).unwrap().note(),
            ux::u7::new(0x6F),
        );
    }

    #[test]
    fn index() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5]).unwrap().index(),
            0xB1,
        );
    }

    #[test]
    fn controller_data() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5]).unwrap().controller_data(),
            0x46105EE5,
        );
    }
}
