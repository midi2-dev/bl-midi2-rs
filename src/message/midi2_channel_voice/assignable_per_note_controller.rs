use crate::{
    message::{helpers as message_helpers, midi2_channel_voice::TYPE_CODE as MIDI2CV_TYPE_CODE},
    result::Result,
    util::{debug, BitOps},
    *,
};

const OP_CODE: u4 = u4::new(0b0001);

#[derive(Clone, PartialEq, Eq)]
pub struct AssignablePerNoteControllerMessage<'a>(&'a [u32]);

debug::message_debug_impl!(AssignablePerNoteControllerMessage);

impl<'a> AssignablePerNoteControllerMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn index(&self) -> u8 {
        self.0[0].octet(3)
    }
    pub fn controller_data(&self) -> u32 {
        self.0[1]
    }
}

impl<'a> Message<'a> for AssignablePerNoteControllerMessage<'a> {
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

impl<'a> Buildable<'a> for AssignablePerNoteControllerMessage<'a> {
    type Builder = AssignablePerNoteControllerBuilder<'a>;
}

impl<'a> GroupedMessage<'a> for AssignablePerNoteControllerMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct AssignablePerNoteControllerBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> AssignablePerNoteControllerBuilder<'a> {
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
    pub fn index(mut self, v: u8) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(3, v);
        }
        self
    }
    pub fn controller_data(mut self, v: u32) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = v;
        }
        self
    }
}

impl<'a> Builder<'a> for AssignablePerNoteControllerBuilder<'a> {
    type Message = AssignablePerNoteControllerMessage<'a>;
    fn build(self) -> Result<AssignablePerNoteControllerMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(AssignablePerNoteControllerMessage(buffer)),
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

impl<'a> GroupedBuilder<'a> for AssignablePerNoteControllerBuilder<'a> {
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
            AssignablePerNoteControllerMessage::builder(&mut random_buffer::<2>())
                .group(u4::new(0x2))
                .channel(u4::new(0x4))
                .note(u7::new(0x6F))
                .index(0xB1)
                .controller_data(0x46105EE5)
                .build(),
            Ok(AssignablePerNoteControllerMessage(&[
                0x4214_6FB1,
                0x46105EE5
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5])
                .unwrap()
                .group(),
            u4::new(0x2),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5])
                .unwrap()
                .channel(),
            u4::new(0x4),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5])
                .unwrap()
                .note(),
            u7::new(0x6F),
        );
    }

    #[test]
    fn index() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5])
                .unwrap()
                .index(),
            0xB1,
        );
    }

    #[test]
    fn controller_data() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5])
                .unwrap()
                .controller_data(),
            0x46105EE5,
        );
    }
}
