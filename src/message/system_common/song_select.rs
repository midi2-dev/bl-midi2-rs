use crate::{
    message::{
        helpers as message_helpers,
        system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
    },
    result::Result,
    util::{BitOps, Truncate},
    *,
};

const OP_CODE: u8 = 0xF3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SongSelectMessage<'a, B: Buffer>(&'a B::Data);

impl<'a> SongSelectMessage<'a, Ump> {
    pub fn song(&self) -> u7 {
        self.0[0].octet(2).truncate()
    }
}

impl<'a> SongSelectMessage<'a, Bytes> {
    pub fn song(&self) -> u7 {
        self.0[1].truncate()
    }
}

impl<'a> Message<'a, Ump> for SongSelectMessage<'a, Ump> {
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

impl<'a> Buildable<'a, Ump> for SongSelectMessage<'a, Ump> {
    type Builder = SongSelectBuilder<'a, Ump>;
}

impl<'a> Buildable<'a, Bytes> for SongSelectMessage<'a, Bytes> {
    type Builder = SongSelectBuilder<'a, Bytes>;
}

impl<'a> Message<'a, Bytes> for SongSelectMessage<'a, Bytes> {
    fn from_data_unchecked(buffer: &'a <Bytes as Buffer>::Data) -> Self {
        Self(buffer)
    }
    fn validate_data(buffer: &'a <Bytes as Buffer>::Data) -> Result<()> {
        system_common::validate_bytes(buffer, OP_CODE, 2)?;
        Ok(())
    }
    fn data(&self) -> &'a <Bytes as Buffer>::Data {
        self.0
    }
}

impl<'a> GroupedMessage<'a> for SongSelectMessage<'a, Ump> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct SongSelectBuilder<'a, B: Buffer>(Result<&'a mut B::Data>);

impl<'a> SongSelectBuilder<'a, Ump> {
    pub fn song(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, v.into());
        }
        self
    }
}

impl<'a> SongSelectBuilder<'a, Bytes> {
    pub fn song(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = v.into();
        }
        self
    }
}

impl<'a> Builder<'a, Ump> for SongSelectBuilder<'a, Ump> {
    type Message = SongSelectMessage<'a, Ump>;
    fn build(self) -> Result<Self::Message> {
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

impl<'a> Builder<'a, Bytes> for SongSelectBuilder<'a, Bytes> {
    type Message = SongSelectMessage<'a, Bytes>;
    fn build(self) -> Result<Self::Message> {
        match self.0 {
            Ok(buffer) => Ok(SongSelectMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
    fn new(buffer: &'a mut [u8]) -> Self {
        if buffer.len() >= 2 {
            message_helpers::clear_buffer(buffer);
            buffer[0] = OP_CODE;
            Self(Ok(buffer))
        } else {
            Self(Err(Error::BufferOverflow))
        }
    }
}

impl<'a> GroupedBuilder<'a> for SongSelectBuilder<'a, Ump> {
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
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            SongSelectMessage::<Ump>::builder(&mut Ump::random_buffer::<1>())
                .group(u4::new(0xA))
                .song(u7::new(0x4F))
                .build(),
            Ok(SongSelectMessage::<Ump>(&[0x1AF3_4F00])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            SongSelectMessage::<Ump>::from_data(&[0x1AF3_4F00])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn song() {
        assert_eq!(
            SongSelectMessage::<Ump>::from_data(&[0x1AF3_4F00])
                .unwrap()
                .song(),
            u7::new(0x4F),
        );
    }

    #[test]
    fn bytes_builder() {
        assert_eq!(
            SongSelectMessage::<Bytes>::builder(&mut Bytes::random_buffer::<3>())
                .song(u7::new(0x4F))
                .build(),
            Ok(SongSelectMessage::<Bytes>(&[0xF3, 0x4F, 0x00])),
        );
    }

    #[test]
    fn bytes_song() {
        assert_eq!(
            SongSelectMessage::<Bytes>::from_data(&[0xF3, 0x4F, 0x00])
                .unwrap()
                .song(),
            u7::new(0x4F),
        );
    }
}
