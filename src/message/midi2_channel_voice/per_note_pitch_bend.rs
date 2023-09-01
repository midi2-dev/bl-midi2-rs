use crate::{
    error::Error,
    message::helpers as message_helpers,
    message::midi2_channel_voice::{helpers as midi2cv_helpers, TYPE_CODE as MIDI2CV_TYPE_CODE},
    result::Result,
    util::debug,
    *,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PerNotePitchBendMessage<'a>(&'a [u32]);

debug::message_debug_impl!(PerNotePitchBendMessage);

const OP_CODE: u4 = u4::new(0b0110);

impl<'a> PerNotePitchBendMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> PerNotePitchBendBuilder {
        PerNotePitchBendBuilder::new(buffer)
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
    pub fn pitch_bend_data(&self) -> u32 {
        self.0[1]
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        midi2cv_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(data, 2)?;
        Ok(Self(data))
    }
}

pub struct PerNotePitchBendBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> PerNotePitchBendBuilder<'a> {
    fn new(buffer: &'a mut [u32]) -> Self {
        match buffer.len() {
            0 | 1 => Self(Err(Error::BufferOverflow)),
            _ => {
                message_helpers::clear_buffer(buffer);
                message_helpers::write_type_to_packet(MIDI2CV_TYPE_CODE, buffer);
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                Self(Ok(buffer))
            }
        }
    }
    pub fn group(&mut self, group: u4) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(group, buffer);
        }
        self
    }
    pub fn channel(&mut self, channel: u4) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(channel, buffer);
        }
        self
    }
    pub fn note(&mut self, v: u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn pitch_bend_data(&mut self, data: u32) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = data;
        }
        self
    }
    pub fn build(&'a self) -> Result<PerNotePitchBendMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(PerNotePitchBendMessage(buffer)),
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
            PerNotePitchBendMessage::builder(&mut random_buffer::<2>())
                .group(u4::new(0x9))
                .channel(u4::new(0x2))
                .note(u7::new(0x76))
                .pitch_bend_data(0x2AD74672)
                .build(),
            Ok(PerNotePitchBendMessage(&[0x4962_7600, 0x2AD74672])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            PerNotePitchBendMessage::from_data(&[0x4962_7600, 0x2AD74672])
                .unwrap()
                .group(),
            u4::new(0x9),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            PerNotePitchBendMessage::from_data(&[0x4962_7600, 0x2AD74672])
                .unwrap()
                .channel(),
            u4::new(0x2),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            PerNotePitchBendMessage::from_data(&[0x4962_7600, 0x2AD74672])
                .unwrap()
                .note(),
            u7::new(0x76),
        );
    }

    #[test]
    fn pitch_bend_data() {
        assert_eq!(
            PerNotePitchBendMessage::from_data(&[0x4962_7600, 0x2AD74672])
                .unwrap()
                .pitch_bend_data(),
            0x2AD74672,
        );
    }
}
