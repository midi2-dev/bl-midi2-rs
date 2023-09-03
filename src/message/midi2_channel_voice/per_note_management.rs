use crate::{
    message::{helpers as message_helpers, midi2_channel_voice::TYPE_CODE as MIDI2CV_TYPE_CODE},
    result::Result,
    util::{debug, BitOps},
    *,
};

const OP_CODE: u4 = u4::new(0b1111);

#[derive(Clone, PartialEq, Eq)]
pub struct PerNoteManagementMessage<'a>(&'a [u32]);

debug::message_debug_impl!(PerNoteManagementMessage);

impl<'a> PerNoteManagementMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> PerNoteManagementBuilder {
        PerNoteManagementBuilder::new(buffer)
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
    pub fn detach(&self) -> bool {
        self.0[0].bit(30)
    }
    pub fn reset(&self) -> bool {
        self.0[0].bit(31)
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        message_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct PerNoteManagementBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> PerNoteManagementBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 1) {
            Ok(()) => {
                message_helpers::clear_buffer(buffer);
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
    pub fn detach(&mut self, v: bool) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_bit(30, v);
        }
        self
    }
    pub fn reset(&mut self, v: bool) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_bit(31, v);
        }
        self
    }
    pub fn build(&'a self) -> Result<PerNoteManagementMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(PerNoteManagementMessage(buffer)),
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
            PerNoteManagementMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0xB))
                .channel(u4::new(0x9))
                .note(u7::new(0x1C))
                .detach(true)
                .reset(true)
                .build(),
            Ok(PerNoteManagementMessage(&[0x4BF9_1C03])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            PerNoteManagementMessage::from_data(&[0x4BF9_1C03])
                .unwrap()
                .group(),
            u4::new(0xB),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            PerNoteManagementMessage::from_data(&[0x4BF9_1C03])
                .unwrap()
                .channel(),
            u4::new(0x9),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            PerNoteManagementMessage::from_data(&[0x4BF9_1C03])
                .unwrap()
                .note(),
            u7::new(0x1C),
        );
    }

    #[test]
    fn detach() {
        assert!(PerNoteManagementMessage::from_data(&[0x4BF9_1C03])
            .unwrap()
            .detach(),);
    }

    #[test]
    fn reset() {
        assert!(PerNoteManagementMessage::from_data(&[0x4BF9_1C03])
            .unwrap()
            .reset(),);
    }
}
