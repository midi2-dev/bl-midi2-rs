use crate::{
    message::{
        helpers as message_helpers, midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE,
    },
    result::Result,
    util::{debug, BitOps, Encode7Bit, Truncate},
    *,
};

const OP_CODE: u4 = u4::new(0b1110);

#[derive(Clone, PartialEq, Eq)]
pub struct PitchBendMessage<'a>(&'a [u32]);

debug::message_debug_impl!(PitchBendMessage);

impl<'a> PitchBendMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn bend(&self) -> u14 {
        u14::from_u7s(&[self.0[0].octet(2).truncate(), self.0[0].octet(3).truncate()])
    }
}

impl<'a> Message<'a> for PitchBendMessage<'a> {
    type Builder = PitchBendBuilder<'a>;
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

impl<'a> GroupedMessage<'a> for PitchBendMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct PitchBendBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> PitchBendBuilder<'a> {
    pub fn channel(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(v, buffer);
        }
        self
    }
    pub fn bend(mut self, v: u14) -> Self {
        if let Ok(buffer) = &mut self.0 {
            let u7s = v.to_u7s();
            buffer[0].set_octet(2, u7s[0].into());
            buffer[0].set_octet(3, u7s[1].into());
        }
        self
    }
}

impl<'a> Builder<'a> for PitchBendBuilder<'a> {
    type Message = PitchBendMessage<'a>;
    fn build(self) -> Result<PitchBendMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(PitchBendMessage(buffer)),
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

impl<'a> GroupedBuilder<'a> for PitchBendBuilder<'a> {
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
            PitchBendMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0x1))
                .channel(u4::new(0xE))
                .bend(u14::new(0x147))
                .build(),
            Ok(PitchBendMessage(&[0x21EE_4702])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            PitchBendMessage::from_data(&[0x21EE_4702]).unwrap().group(),
            u4::new(0x1),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            PitchBendMessage::from_data(&[0x21EE_4702])
                .unwrap()
                .channel(),
            u4::new(0xE),
        );
    }

    #[test]
    fn bend() {
        assert_eq!(
            PitchBendMessage::from_data(&[0x21EE_4702]).unwrap().bend(),
            u14::new(0x147)
        );
    }
}
