use crate::{
    *,
    message::{
        helpers as message_helpers,
        midi2_channel_voice::TYPE_CODE as MIDI2CV_TYPE_CODE,
    },
    result::Result,
    util::debug,
};

const OP_CODE: u4 = u4::new(0b1101);

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelPressureMessage<'a>(&'a [u32]);

debug::message_debug_impl!(ChannelPressureMessage);

impl<'a> ChannelPressureMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> ChannelPressureBuilder {
        ChannelPressureBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn channel_pressure_data(&self) -> u32 {
        self.0[1]
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(data, 2)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct ChannelPressureBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ChannelPressureBuilder<'a> {
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
    pub fn channel_pressure_data(&mut self, v: u32) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = v;
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
            ChannelPressureMessage::builder(&mut [0x0, 0x0])
                .group(u4::new(0xE))
                .channel(u4::new(0xD))
                .channel_pressure_data(0xDE0DE0F2)
                .build(),
            Ok(ChannelPressureMessage(&[0x4EDD_0000, 0xDE0D_E0F2])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x4EDD_0000, 0xDE0D_E0F2])
                .unwrap()
                .group(),
            u4::new(0xE),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x4EDD_0000, 0xDE0D_E0F2])
                .unwrap()
                .channel(),
            u4::new(0xD),
        );
    }

    #[test]
    fn channel_pressure_data() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x4EDD_0000, 0xDE0D_E0F2])
                .unwrap()
                .channel_pressure_data(),
            0xDE0DE0F2,
        );
    }
}
