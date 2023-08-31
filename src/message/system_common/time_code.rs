use crate::{
    message::{
        helpers as message_helpers,
        system_common::{
            TYPE_CODE as SYSTEM_COMMON_TYPE_CODE,
            self,
        },
    },
    result::Result,
    util::{debug, BitOps, Truncate},
};

const OP_CODE: u8 = 0xF1;

#[derive(Clone, PartialEq, Eq)]
pub struct TimeCodeMessage<'a>(&'a [u32]);

debug::message_debug_impl!(TimeCodeMessage);

impl<'a> TimeCodeMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> TimeCodeBuilder {
        TimeCodeBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn time_code(&self) -> ux::u7 {
        self.0[0].octet(2).truncate()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        system_common::validate_packet(data, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct TimeCodeBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> TimeCodeBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        match system_common::validate_buffer_size(buffer) {
            Ok(()) => {
                system_common::write_op_code_to_packet(buffer, OP_CODE);
                message_helpers::write_type_to_packet(SYSTEM_COMMON_TYPE_CODE, buffer);
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
    pub fn time_code(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, v.into());
        }
        self
    }
    pub fn build(&'a self) -> Result<TimeCodeMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(TimeCodeMessage(buffer)),
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
            TimeCodeMessage::builder(&mut [0x0])
                .group(ux::u4::new(0x5))
                .time_code(ux::u7::new(0x5F))
                .build(),
            Ok(TimeCodeMessage(&[0x15F1_5F00])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TimeCodeMessage::from_data(&[0x15F1_5F00]).unwrap().group(),
            ux::u4::new(0x5),
        );
    }

    #[test]
    fn time_code() {
        assert_eq!(
            TimeCodeMessage::from_data(&[0x15F1_5F00]).unwrap().time_code(),
            ux::u7::new(0x5F),
        );
    }
}
