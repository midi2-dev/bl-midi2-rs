use crate::{
    *,
    message::{
        helpers as message_helpers,
        midi2_channel_voice::{
            attribute, helpers as midi2cv_helpers, TYPE_CODE as MIDI2CV_TYPE_CODE,
        },
    },
    result::Result,
    util::debug,
};

const OP_CODE: u4 = u4::new(0b1000);

#[derive(Clone, PartialEq, Eq)]
pub struct NoteOffMessage<'a>(&'a [u32]);

debug::message_debug_impl!(NoteOffMessage);

impl<'a> NoteOffMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> NoteOffBuilder {
        NoteOffBuilder::new(buffer)
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
    pub fn velocity(&self) -> u16 {
        midi2cv_helpers::note_velocity_from_packet(self.0)
    }
    pub fn attribute(&self) -> Option<attribute::Attribute> {
        attribute::from_ump(self.0)
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        message_helpers::validate_buffer_size(data, 2)?;
        attribute::validate_ump(data)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct NoteOffBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> NoteOffBuilder<'a> {
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
    pub fn note(&mut self, v: u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn velocity(&mut self, v: u16) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_note_velocity_to_packet(v, buffer);
        }
        self
    }
    pub fn attribute(&mut self, v: attribute::Attribute) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            attribute::write_attribute(buffer, Some(v));
        }
        self
    }
    pub fn build(&'a self) -> Result<NoteOffMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(NoteOffMessage(buffer)),
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
            NoteOffMessage::builder(&mut [0x0, 0x0])
                .group(u4::new(0x2))
                .channel(u4::new(0x4))
                .note(u7::new(0x4E))
                .velocity(0x9DE6)
                .attribute(attribute::Attribute::ManufacturerSpecific(0xCC6E))
                .build(),
            Ok(NoteOffMessage(&[0x4284_4E01, 0x9DE6_CC6E]))
        );
    }

    #[test]
    fn builder_no_attribute() {
        assert_eq!(
            NoteOffMessage::builder(&mut [0x0, 0x0])
                .group(u4::new(0x2))
                .channel(u4::new(0x4))
                .note(u7::new(0x4E))
                .velocity(0x9DE6)
                .build(),
            Ok(NoteOffMessage(&[0x4284_4E00, 0x9DE6_0000]))
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x4284_4E01, 0x9DE6_CC6E])
                .unwrap()
                .group(),
            u4::new(0x2),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x4284_4E01, 0x9DE6_CC6E])
                .unwrap()
                .channel(),
            u4::new(0x4),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x4284_4E01, 0x9DE6_CC6E])
                .unwrap()
                .note(),
            u7::new(0x4E),
        );
    }

    #[test]
    fn volocity() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x4284_4E01, 0x9DE6_CC6E])
                .unwrap()
                .velocity(),
            0x9DE6,
        );
    }

    #[test]
    fn attribute() {
        assert_eq!(
            NoteOffMessage::from_data(&[0x4284_4E01, 0x9DE6_CC6E])
                .unwrap()
                .attribute(),
            Some(attribute::Attribute::ManufacturerSpecific(0xCC6E)),
        );
    }
}
