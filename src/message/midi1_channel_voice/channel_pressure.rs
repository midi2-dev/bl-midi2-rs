use crate::{
    message::{
        midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE,
        helpers as message_helpers,
    },
    result::Result,
    util::debug,
};

const OP_CODE: ux::u4 = ux::u4::new(0b1101);

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelPressureMessage<'a>(&'a [u32]);

debug::message_debug_impl!(ChannelPressureMessage);

impl<'a> ChannelPressureMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> ChannelPressureBuilder {
        ChannelPressureBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> ux::u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn pressure(&self) -> ux::u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct ChannelPressureBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ChannelPressureBuilder<'a> {
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
    pub fn pressure(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn build(&'a self) -> Result<ChannelPressureMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(ChannelPressureMessage(buffer)),
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
            ChannelPressureMessage::builder(&mut [0x0])
                .group(ux::u4::new(0xF))
                .channel(ux::u4::new(0x6))
                .pressure(ux::u7::new(0x09))
                .build(),
            Ok(ChannelPressureMessage(&[0x2FD6_0900])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900]).unwrap().group(),
            ux::u4::new(0xF),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900]).unwrap().channel(),
            ux::u4::new(0x6),
        );
    }

    #[test]
    fn pressure() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900]).unwrap().pressure(),
            ux::u7::new(0x09),
        );
    }
}
