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
    pub fn builder(buffer: &mut [u32]) -> ProgramChangeBuilder {
        ProgramChangeBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn program(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct ProgramChangeBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ProgramChangeBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 1) {
            Ok(()) => {
                message_helpers::clear_buffer(buffer);
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI1_CHANNEL_VOICE_TYPE, buffer);
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
    pub fn program(&mut self, v: u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn build(&'a self) -> Result<ProgramChangeMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(ProgramChangeMessage(buffer)),
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
