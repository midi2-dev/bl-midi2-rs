use crate::{
    *,
    error::Error,
    message::helpers as message_helpers,
    message::midi2_channel_voice::{helpers as midi2cv_helpers, TYPE_CODE as MIDI2CV_TYPE_CODE},
    result::Result,
    util::{debug, BitOps, Truncate},
};

#[derive(Clone, PartialEq, Eq)]
pub struct ControlChangeMessage<'a>(&'a [u32]);

debug::message_debug_impl!(ControlChangeMessage);

const OP_CODE: u4 = u4::new(0b1011);

impl<'a> ControlChangeMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> ControlChangeBuilder {
        ControlChangeBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn index(&self) -> u7 {
        self.0[0].octet(2).truncate()
    }
    pub fn control_change_data(&self) -> u32 {
        self.0[1]
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        midi2cv_helpers::validate_packet(data, MIDI2CV_TYPE_CODE, OP_CODE)?;
        if data.len() < 2 {
            return Err(Error::BufferOverflow);
        }
        Ok(Self(data))
    }
}

pub struct ControlChangeBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ControlChangeBuilder<'a> {
    fn new(buffer: &'a mut [u32]) -> Self {
        match buffer.len() {
            0 | 1 => Self(Err(Error::BufferOverflow)),
            _ => {
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
    pub fn index(&mut self, index: u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, index.into());
        }
        self
    }
    pub fn control_change_data(&mut self, data: u32) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = data;
        }
        self
    }
    pub fn build(&'a self) -> Result<ControlChangeMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(ControlChangeMessage(buffer)),
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
            ControlChangeMessage::builder(&mut [0x0, 0x0])
                .group(u4::new(0x3))
                .channel(u4::new(0x9))
                .index(u7::new(0x30))
                .control_change_data(0x2468_1012)
                .build(),
            Ok(ControlChangeMessage(&[0x43B9_3000, 0x2468_1012]))
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x43B9_3000, 0x2468_1012])
                .unwrap()
                .group(),
            u4::new(0x3),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x43B9_3000, 0x2468_1012])
                .unwrap()
                .channel(),
            u4::new(0x9),
        );
    }

    #[test]
    fn index() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x43B9_3000, 0x2468_1012])
                .unwrap()
                .index(),
            u7::new(0x30),
        );
    }

    #[test]
    fn control_change_data() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x43B9_3000, 0x2468_1012])
                .unwrap()
                .control_change_data(),
            0x2468_1012,
        );
    }
}
