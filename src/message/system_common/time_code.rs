use crate::{
    message::{
        helpers as message_helpers,
        system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
    },
    result::Result,
    util::{debug, BitOps, Truncate},
    *,
};

const OP_CODE: u8 = 0xF1;

#[derive(Clone, PartialEq, Eq)]
pub struct TimeCodeMessage<'a>(&'a [u32]);

debug::message_debug_impl!(TimeCodeMessage);

impl<'a> TimeCodeMessage<'a> {
    pub fn time_code(&self) -> u7 {
        self.0[0].octet(2).truncate()
    }
}

impl<'a> Message<'a> for TimeCodeMessage<'a> {
    type Builder = TimeCodeBuilder<'a>;
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        Self(data)
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        system_common::validate_packet(buffer, OP_CODE)
    }
    fn data(&self) -> &'a [u32] {
        self.0
    }
}

impl<'a> GroupedMessage<'a> for TimeCodeMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct TimeCodeBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> TimeCodeBuilder<'a> {
    pub fn time_code(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, v.into());
        }
        self
    }
}

impl<'a> Builder<'a> for TimeCodeBuilder<'a> {
    type Message = TimeCodeMessage<'a>;
    fn new(buffer: &'a mut [u32]) -> Self {
        match system_common::validate_buffer_size(buffer) {
            Ok(()) => {
                message_helpers::clear_buffer(buffer);
                system_common::write_op_code_to_packet(buffer, OP_CODE);
                message_helpers::write_type_to_packet(SYSTEM_COMMON_TYPE_CODE, buffer);
                Self(Ok(buffer))
            }
            Err(e) => Self(Err(e)),
        }
    }
    fn build(self) -> Result<TimeCodeMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(TimeCodeMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl<'a> GroupedBuilder<'a> for TimeCodeBuilder<'a> {
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
            TimeCodeMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0x5))
                .time_code(u7::new(0x5F))
                .build(),
            Ok(TimeCodeMessage(&[0x15F1_5F00])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TimeCodeMessage::from_data(&[0x15F1_5F00]).unwrap().group(),
            u4::new(0x5),
        );
    }

    #[test]
    fn time_code() {
        assert_eq!(
            TimeCodeMessage::from_data(&[0x15F1_5F00])
                .unwrap()
                .time_code(),
            u7::new(0x5F),
        );
    }
}
