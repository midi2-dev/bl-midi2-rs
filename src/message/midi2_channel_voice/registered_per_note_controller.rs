use crate::{
    message::{
        helpers as message_helpers,
        midi2_channel_voice::{controller, TYPE_CODE as MIDI2CV_TYPE_CODE},
    },
    result::Result,
    util::{debug, BitOps},
    *,
};

const TYPE_CODE: u4 = u4::new(0b0000);

#[derive(Clone, PartialEq, Eq)]
pub struct RegisteredPerNoteControllerMessage<'a>(&'a [u32]);

debug::message_debug_impl!(RegisteredPerNoteControllerMessage);

impl<'a> RegisteredPerNoteControllerMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn controller(&self) -> controller::Controller {
        controller::from_index_and_data(self.0[0].octet(3), self.0[1])
    }
}

impl<'a> Message<'a, Ump> for RegisteredPerNoteControllerMessage<'a> {
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI2CV_TYPE_CODE, TYPE_CODE)?;
        message_helpers::validate_buffer_size(buffer, 2)?;
        controller::validate_index(buffer[0].octet(3))?;
        Ok(())
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Self(buffer)
    }
}

impl<'a> Buildable<'a, Ump> for RegisteredPerNoteControllerMessage<'a> {
    type Builder = RegisteredPerNoteControllerBuilder<'a>;
}

impl<'a> GroupedMessage<'a> for RegisteredPerNoteControllerMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct RegisteredPerNoteControllerBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> RegisteredPerNoteControllerBuilder<'a> {
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
    pub fn controller(mut self, v: controller::Controller) -> Self {
        if let Ok(buffer) = &mut self.0 {
            let (index, data) = controller::to_index_and_data(v);
            buffer[0].set_octet(3, index);
            buffer[1] = data;
        }
        self
    }
}

impl<'a> Builder<'a, Ump> for RegisteredPerNoteControllerBuilder<'a> {
    type Message = RegisteredPerNoteControllerMessage<'a>;
    fn build(self) -> Result<RegisteredPerNoteControllerMessage<'a>> {
        match self.0 {
            Ok(buffer) => {
                controller::validate_index(buffer[0].octet(3))?;
                Ok(RegisteredPerNoteControllerMessage(buffer))
            }
            Err(e) => Err(e.clone()),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 2) {
            Ok(()) => {
                message_helpers::clear_buffer(&mut buffer[..2]);
                message_helpers::write_op_code_to_packet(TYPE_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI2CV_TYPE_CODE, buffer);
                Self(Ok(&mut buffer[..2]))
            }
            Err(e) => Self(Err(e)),
        }
    }
}

impl<'a> GroupedBuilder<'a> for RegisteredPerNoteControllerBuilder<'a> {
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
            RegisteredPerNoteControllerMessage::builder(&mut Ump::random_buffer::<2>())
                .group(u4::new(0x4))
                .channel(u4::new(0x5))
                .note(u7::new(0x6C))
                .controller(controller::Controller::Volume(0xE1E35E92))
                .build(),
            Ok(RegisteredPerNoteControllerMessage(&[
                0x4405_6C07,
                0xE1E35E92
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            RegisteredPerNoteControllerMessage::from_data(&[0x4405_6C07, 0xE1E35E92])
                .unwrap()
                .group(),
            u4::new(0x4),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            RegisteredPerNoteControllerMessage::from_data(&[0x4405_6C07, 0xE1E35E92])
                .unwrap()
                .channel(),
            u4::new(0x5),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            RegisteredPerNoteControllerMessage::from_data(&[0x4405_6C07, 0xE1E35E92])
                .unwrap()
                .note(),
            u7::new(0x6C),
        );
    }

    #[test]
    fn controller() {
        assert_eq!(
            RegisteredPerNoteControllerMessage::from_data(&[0x4405_6C07, 0xE1E35E92])
                .unwrap()
                .controller(),
            controller::Controller::Volume(0xE1E35E92),
        );
    }
}
