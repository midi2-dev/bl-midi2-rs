use crate::{
    *,
    message::{
        helpers as message_helpers,
        midi2_channel_voice::TYPE_CODE as MIDI2CV_TYPE_CODE,
    },
    result::Result,
    util::debug,
};

const OP_CODE: u4 = u4::new(0b1110);

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelPitchBendMessage<'a>(&'a [u32]);

debug::message_debug_impl!(ChannelPitchBendMessage);

impl<'a> ChannelPitchBendMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> ChannelPitchBendBuilder {
        ChannelPitchBendBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn pitch_bend_data(&self) -> u32 {
        self.0[1]
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(data, 2)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct ChannelPitchBendBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ChannelPitchBendBuilder<'a> {
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
    pub fn pitch_bend_data(&mut self, v: u32) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = v;
        }
        self
    }
    pub fn build(&'a self) -> Result<ChannelPitchBendMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(ChannelPitchBendMessage(buffer)),
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
            ChannelPitchBendMessage::builder(&mut [0x0, 0x0])
                .group(u4::new(0xB))
                .channel(u4::new(0x9))
                .pitch_bend_data(0x08306AF8)
                .build(),
            Ok(ChannelPitchBendMessage(&[0x4BE9_0000, 0x0830_6AF8])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelPitchBendMessage::from_data(&[0x4BE9_0000, 0x0830_6AF8])
                .unwrap()
                .group(),
            u4::new(0xB),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ChannelPitchBendMessage::from_data(&[0x4BE9_0000, 0x0830_6AF8])
                .unwrap()
                .channel(),
            u4::new(0x9),
        );
    }

    #[test]
    fn pitch_bend_data() {
        assert_eq!(
            ChannelPitchBendMessage::from_data(&[0x4BE9_0000, 0x0830_6AF8])
                .unwrap()
                .pitch_bend_data(),
            0x0830_6AF8,
        );
    }
}
