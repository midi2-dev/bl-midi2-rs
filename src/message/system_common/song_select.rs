use crate::{
    message::{
        helpers as message_helpers,
        system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
    },
    result::Result,
    util::{debug, BitOps, Truncate},
    *,
};

const OP_CODE: u8 = 0xF3;

#[derive(Clone, PartialEq, Eq)]
pub struct SongSelectMessage<'a>(&'a [u32]);

debug::message_debug_impl!(SongSelectMessage);

impl<'a> SongSelectMessage<'a> {
    pub fn song(&self) -> u7 {
        self.0[0].octet(2).truncate()
    }
}

impl<'a> Message<'a> for SongSelectMessage<'a> {
    type Builder = SongSelectBuilder<'a>;
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        Self(data)
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        system_common::validate_packet(buffer, OP_CODE)?;
        Ok(())
    }
    fn data(&self) -> &'a [u32] {
        self.0
    }
}

impl<'a> GroupedMessage<'a> for SongSelectMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct SongSelectBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> SongSelectBuilder<'a> {
    pub fn song(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, v.into());
        }
        self
    }
}

impl<'a> Builder<'a> for SongSelectBuilder<'a> {
    type Message = SongSelectMessage<'a>;
    fn build(self) -> Result<SongSelectMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(SongSelectMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
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
}

impl<'a> GroupedBuilder<'a> for SongSelectBuilder<'a> {
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
            SongSelectMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0xA))
                .song(u7::new(0x4F))
                .build(),
            Ok(SongSelectMessage(&[0x1AF3_4F00])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            SongSelectMessage::from_data(&[0x1AF3_4F00])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn song() {
        assert_eq!(
            SongSelectMessage::from_data(&[0x1AF3_4F00]).unwrap().song(),
            u7::new(0x4F),
        );
    }
}
