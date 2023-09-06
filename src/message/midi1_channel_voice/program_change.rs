use crate::{
    message::{
        helpers as message_helpers, midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE,
    },
    result::Result,
    util::debug,
    *,
};

const OP_CODE: u4 = u4::new(0b1100);

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramChangeMessage<'a>(&'a [u32]);

debug::message_debug_impl!(ProgramChangeMessage);

impl<'a> ProgramChangeMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn program(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
}

impl<'a> Message<'a> for ProgramChangeMessage<'a> {
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Self(buffer)
    }
}

impl<'a> Buildable<'a> for ProgramChangeMessage<'a> {
    type Builder = ProgramChangeBuilder<'a>;
}

impl<'a> GroupedMessage<'a> for ProgramChangeMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct ProgramChangeBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ProgramChangeBuilder<'a> {
    pub fn group(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(v, buffer);
        }
        self
    }
    pub fn channel(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(v, buffer);
        }
        self
    }
    pub fn program(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
}

impl<'a> Builder<'a> for ProgramChangeBuilder<'a> {
    type Message = ProgramChangeMessage<'a>;
    fn build(self) -> Result<ProgramChangeMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(ProgramChangeMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 1) {
            Ok(()) => {
                message_helpers::clear_buffer(&mut buffer[..1]);
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI1_CHANNEL_VOICE_TYPE, buffer);
                Self(Ok(&mut buffer[..1]))
            }
            Err(e) => Self(Err(e)),
        }
    }
}

impl<'a> GroupedBuilder<'a> for ProgramChangeBuilder<'a> {
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
            ProgramChangeMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0x4))
                .channel(u4::new(0x7))
                .program(u7::new(0x63))
                .build(),
            Ok(ProgramChangeMessage(&[0x24C7_6300])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x24C7_6300])
                .unwrap()
                .group(),
            u4::new(0x4),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x24C7_6300])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn program() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x24C7_6300])
                .unwrap()
                .program(),
            u7::new(0x63),
        );
    }
}
