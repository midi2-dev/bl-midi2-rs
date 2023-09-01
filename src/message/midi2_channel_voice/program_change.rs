use crate::{
    message::{
        helpers as message_helpers,
        midi2_channel_voice::TYPE_CODE as MIDI2CV_TYPE_CODE,
    },
    result::Result,
    util::{debug, BitOps, Encode7Bit, Truncate},
};

const OP_CODE: ux::u4 = ux::u4::new(0b1100);

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramChangeMessage<'a>(&'a [u32]);

debug::message_debug_impl!(ProgramChangeMessage);

impl<'a> ProgramChangeMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> ProgramChangeBuilder {
        ProgramChangeBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> ux::u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn program(&self) -> ux::u7 {
        self.0[1].octet(0).truncate()
    }
    pub fn bank(&self) -> Option<ux::u14> {
        if self.0[0].bit(31) {
            Some(ux::u14::from_u7s(&[self.0[1].octet(2), self.0[1].octet(3)]))
        } else {
            None
        }
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(data, 2)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct ProgramChangeBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ProgramChangeBuilder<'a> {
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
    pub fn program(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1].set_octet(0, v.into());
        }
        self
    }
    pub fn bank(&mut self, v: ux::u14) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            let u7s = v.to_u7s();
            buffer[0].set_bit(31, true);
            buffer[1].set_octet(2, u7s[0].into());
            buffer[1].set_octet(3, u7s[1].into());
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

    #[test]
    fn builder() {
        assert_eq!(
            ProgramChangeMessage::builder(&mut [0x0, 0x0])
                .group(ux::u4::new(0xF))
                .channel(ux::u4::new(0xE))
                .program(ux::u7::new(0x75))
                .bank(ux::u14::new(0x1F5E))
                .build(),
            Ok(ProgramChangeMessage(&[0x4FCE_0001, 0x7500_5E3E])),
        );
    }

    #[test]
    fn builder_no_bank() {
        assert_eq!(
            ProgramChangeMessage::builder(&mut [0x0, 0x0])
                .group(ux::u4::new(0xF))
                .channel(ux::u4::new(0xE))
                .program(ux::u7::new(0x75))
                .build(),
            Ok(ProgramChangeMessage(&[0x4FCE_0000, 0x7500_0000])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0001, 0x7500_5E3E])
                .unwrap()
                .group(),
            ux::u4::new(0xF),
        )
    }

    #[test]
    fn channel() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0001, 0x7500_5E3E])
                .unwrap()
                .channel(),
            ux::u4::new(0xE),
        )
    }

    #[test]
    fn program() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0001, 0x7500_5E3E])
                .unwrap()
                .program(),
            ux::u7::new(0x75),
        )
    }

    #[test]
    fn bank() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0001, 0x7500_5E3E])
                .unwrap()
                .bank(),
            Some(ux::u14::new(0x1F5E)),
        )
    }

    #[test]
    fn no_bank() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0000, 0x7500_0000])
                .unwrap()
                .bank(),
            None,
        )
    }
}
