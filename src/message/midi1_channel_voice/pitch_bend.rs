use crate::{
    *,
    message::{
        midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE,
        helpers as message_helpers,
    },
    result::Result,
    util::{Encode7Bit, BitOps, Truncate, debug},
};

const OP_CODE: u4 = u4::new(0b1110);

#[derive(Clone, PartialEq, Eq)]
pub struct PitchBendMessage<'a>(&'a [u32]);

debug::message_debug_impl!(PitchBendMessage);

impl<'a> PitchBendMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> PitchBendBuilder {
        PitchBendBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn bend(&self) -> u14 {
        u14::from_u7s(&[
            self.0[0].octet(2).truncate(),
            self.0[0].octet(3).truncate(),
        ])
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct PitchBendBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> PitchBendBuilder<'a> {
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
    pub fn bend(&mut self, v: u14) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            let u7s = v.to_u7s();
            buffer[0].set_octet(2, u7s[0].into());
            buffer[0].set_octet(3, u7s[1].into());
        }
        self
    }
    pub fn build(&'a self) -> Result<PitchBendMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(PitchBendMessage(buffer)),
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
            PitchBendMessage::builder(&mut [0x0])
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
            PitchBendMessage::from_data(&[0x21EE_4702]).unwrap().channel(),
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
