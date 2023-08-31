use crate::{
    message::{
        helpers as message_helpers,
        system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
    },
    result::Result,
    util::{debug, BitOps, Truncate},
};

const OP_CODE: u8 = 0xF3;

#[derive(Clone, PartialEq, Eq)]
pub struct SongSelectMessage<'a>(&'a [u32]);

debug::message_debug_impl!(SongSelectMessage);

impl<'a> SongSelectMessage<'a> {
    pub fn builder(buffer: &mut [u32]) -> SongSelectBuilder {
        SongSelectBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn song(&self) -> ux::u7 {
        self.0[0].octet(2).truncate()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        system_common::validate_packet(data, OP_CODE)?;
        Ok(Self(data))
    }
}

#[derive(PartialEq, Eq)]
pub struct SongSelectBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> SongSelectBuilder<'a> {
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
    pub fn song(&mut self, v: ux::u7) -> &mut Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, v.into());
        }
        self
    }
    pub fn build(&'a self) -> Result<SongSelectMessage<'a>> {
        match &self.0 {
            Ok(buffer) => Ok(SongSelectMessage(buffer)),
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
            SongSelectMessage::builder(&mut [0x0])
                .group(ux::u4::new(0xA))
                .song(ux::u7::new(0x4F))
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
            ux::u4::new(0xA),
        );
    }

    #[test]
    fn song() {
        assert_eq!(
            SongSelectMessage::from_data(&[0x1AF3_4F00]).unwrap().song(),
            ux::u7::new(0x4F),
        );
    }
}
