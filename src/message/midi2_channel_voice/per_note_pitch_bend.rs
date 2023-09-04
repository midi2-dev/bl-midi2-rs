use crate::{
    message::helpers as message_helpers,
    message::midi2_channel_voice::TYPE_CODE as MIDI2CV_TYPE_CODE, result::Result, util::debug, *,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PerNotePitchBendMessage<'a>(&'a [u32]);

debug::message_debug_impl!(PerNotePitchBendMessage);

const TYPE_CODE: u4 = u4::new(0b0110);

impl<'a> PerNotePitchBendMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn pitch_bend_data(&self) -> u32 {
        self.0[1]
    }
}

impl<'a> Message<'a> for PerNotePitchBendMessage<'a> {
    type Builder = PerNotePitchBendBuilder<'a>;
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI2CV_TYPE_CODE, TYPE_CODE)?;
        message_helpers::validate_buffer_size(buffer, 2)?;
        Ok(())
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Self(buffer)
    }
}

impl<'a> GroupedMessage<'a> for PerNotePitchBendMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

pub struct PerNotePitchBendBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> PerNotePitchBendBuilder<'a> {
    pub fn channel(mut self, channel: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(channel, buffer);
        }
        self
    }
    pub fn note(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn pitch_bend_data(mut self, data: u32) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = data;
        }
        self
    }
}

impl<'a> Builder<'a> for PerNotePitchBendBuilder<'a> {
    type Message = PerNotePitchBendMessage<'a>;
    fn build(self) -> Result<PerNotePitchBendMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(PerNotePitchBendMessage(buffer)),
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

impl<'a> GroupedBuilder<'a> for PerNotePitchBendBuilder<'a> {
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
