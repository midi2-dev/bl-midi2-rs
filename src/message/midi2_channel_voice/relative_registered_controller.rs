use crate::{
    message::{
        helpers as message_helpers,
        midi2_channel_voice::{
            TYPE_CODE as MIDI2CV_TYPE_CODE,
            helpers as midi2cv_helpers,
        },
    },
    result::Result,
    util::debug,
};

const TYPE_CODE: ux::u4 = ux::u4::new(0b0100);

#[derive(Clone, PartialEq, Eq)]
pub struct RelativeRegisteredControllerMessage<'a>(&'a [u32]);

debug::message_debug_impl!(RelativeRegisteredControllerMessage);

impl<'a> RelativeRegisteredControllerMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> RelativeRegisteredControllerBuilder {
        RelativeRegisteredControllerBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> ux::u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn bank(&self) -> ux::u7 {
        midi2cv_helpers::controller_bank_from_packet(self.0)
    }
    pub fn index(&self) -> ux::u7 {
        midi2cv_helpers::controller_index_from_packet(self.0)
    }
    pub fn controller_data(&self) -> u32 {
        midi2cv_helpers::controller_data_from_packet(self.0)
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, TYPE_CODE)?;
        midi2cv_helpers::validate_controller_message_buffer_size(data)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct RelativeRegisteredControllerBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> RelativeRegisteredControllerBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        match midi2cv_helpers::validate_controller_message_buffer_size(buffer) {
            Ok(()) => {
                message_helpers::write_op_code_to_packet(TYPE_CODE, buffer);
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
    pub fn bank(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_controller_bank_to_packet(v, buffer);
        }
        self
    }
    pub fn index(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_controller_index_to_packet(v, buffer);
        }
        self
    }
    pub fn controller_data(&mut self, v: u32) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_controller_data_to_packet(v, buffer);
        }
        self
    }
    pub fn build(&'a self) -> Result<RelativeRegisteredControllerMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(RelativeRegisteredControllerMessage(buffer)),
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
            RelativeRegisteredControllerMessage::builder(&mut [0x0, 0x0])
                .group(ux::u4::new(0x1))
                .channel(ux::u4::new(0xE))
                .bank(ux::u7::new(0x45))
                .index(ux::u7::new(0x02))
                .controller_data(0xAF525908)
                .build(),
            Ok(RelativeRegisteredControllerMessage(&[0x414E_4502, 0xAF525908])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            RelativeRegisteredControllerMessage::from_data(&[0x414E_4502, 0xAF525908]).unwrap().group(),
            ux::u4::new(0x1),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            RelativeRegisteredControllerMessage::from_data(&[0x414E_4502, 0xAF525908]).unwrap().channel(),
            ux::u4::new(0xE),
        );
    }

    #[test]
    pub fn bank() {
        assert_eq!(
            RelativeRegisteredControllerMessage::from_data(&[0x414E_4502, 0xAF525908]).unwrap().bank(),
            ux::u7::new(0x45),
        );
    }

    #[test]
    pub fn index() {
        assert_eq!(
            RelativeRegisteredControllerMessage::from_data(&[0x414E_4502, 0xAF525908]).unwrap().index(),
            ux::u7::new(0x02),
        );
    }

    #[test]
    pub fn controller_data() {
        assert_eq!(
            RelativeRegisteredControllerMessage::from_data(&[0x414E_4502, 0xAF525908]).unwrap().controller_data(),
            0xAF525908,
        );
    }
}
