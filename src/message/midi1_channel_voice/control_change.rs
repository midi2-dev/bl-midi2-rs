use crate::{
    message::{
        midi1_channel_voice::{
            TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE,
            helpers as midi1cv_helpers,
        },
        helpers as message_helpers,
    },
    result::Result,
    util::debug,
};

const OP_CODE: ux::u4 = ux::u4::new(0b1011);

#[derive(Clone, PartialEq, Eq)]
pub struct ControlChangeMessage<'a>(&'a [u32]);

debug::message_debug_impl!(ControlChangeMessage);

impl<'a> ControlChangeMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> ControlChangeBuilder {
        ControlChangeBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> ux::u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn control(&self) -> ux::u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn control_data(&self) -> ux::u7 {
        midi1cv_helpers::note_velocity_from_packet(self.0)
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct ControlChangeBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ControlChangeBuilder<'a> {
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
    pub fn control(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn control_data(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            midi1cv_helpers::write_note_velocity_to_packet(v, buffer);
        }
        self
    }
    pub fn build(&'a self) -> Result<ControlChangeMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(ControlChangeMessage(buffer)),
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
            ControlChangeMessage::builder(&mut [0x0])
                .group(ux::u4::new(0xA))
                .channel(ux::u4::new(0x7))
                .control(ux::u7::new(0x36))
                .control_data(ux::u7::new(0x37))
                .build(),
            Ok(ControlChangeMessage(&[0x2AB7_3637])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x2AB7_3637]).unwrap().group(),
            ux::u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x2AB7_3637]).unwrap().channel(),
            ux::u4::new(0x7),
        );
    }

    #[test]
    fn control() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x2AB7_3637]).unwrap().control(),
            ux::u7::new(0x36),
        );
    }

    #[test]
    fn control_data() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x2AB7_3637]).unwrap().control_data(),
            ux::u7::new(0x37),
        );
    }
}
