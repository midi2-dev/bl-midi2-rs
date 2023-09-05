use crate::{
    message::{
        helpers as message_helpers,
        system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
    },
    result::Result,
    util::{BitOps, Encode7Bit},
    *,
};

const OP_CODE: u8 = 0xF2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SongPositionPointerMessage<'a, B: Buffer>(&'a B::Data);

impl<'a> SongPositionPointerMessage<'a, Ump> {
    pub fn position(&self) -> u14 {
        u14::from_u7s(&[self.0[0].octet(2), self.0[0].octet(3)])
    }
}

impl<'a> SongPositionPointerMessage<'a, Bytes> {
    pub fn position(&self) -> u14 {
        u14::from_u7s(&[self.0[1], self.0[2]])
    }
}

impl<'a> Message<'a, Ump> for SongPositionPointerMessage<'a, Ump> {
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        Self(data)
    }
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        system_common::validate_packet(buffer, OP_CODE)?;
        Ok(())
    }
}

impl<'a> Buildable<'a, Ump> for SongPositionPointerMessage<'a, Ump> {
    type Builder = SongPositionPointerBuilder<'a, Ump>;
}

impl<'a> Buildable<'a, Bytes> for SongPositionPointerMessage<'a, Bytes> {
    type Builder = SongPositionPointerBuilder<'a, Bytes>;
}

impl<'a> Message<'a, Bytes> for SongPositionPointerMessage<'a, Bytes> {
    fn data(&self) -> &'a <Bytes as Buffer>::Data {
        self.0
    }
    fn validate_data(buffer: &'a <Bytes as Buffer>::Data) -> Result<()> {
        system_common::validate_bytes(buffer, OP_CODE, 3)?;
        Ok(())
    }
    fn from_data_unchecked(buffer: &'a <Bytes as Buffer>::Data) -> Self {
        Self(buffer)
    }
}

impl<'a> GroupedMessage<'a> for SongPositionPointerMessage<'a, Ump> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct SongPositionPointerBuilder<'a, B: Buffer>(Result<&'a mut B::Data>);

impl<'a> SongPositionPointerBuilder<'a, Ump> {
    pub fn position(mut self, v: u14) -> Self {
        if let Ok(buffer) = &mut self.0 {
            let u7s = v.to_u7s();
            buffer[0].set_octet(2, u7s[0].into());
            buffer[0].set_octet(3, u7s[1].into());
        }
        self
    }
}

impl<'a> SongPositionPointerBuilder<'a, Bytes> {
    pub fn position(mut self, v: u14) -> Self {
        if let Ok(buffer) = &mut self.0 {
            let u7s = v.to_u7s();
            buffer[1] = u7s[0].into();
            buffer[2] = u7s[1].into();
        }
        self
    }
}

impl<'a> Builder<'a, Ump> for SongPositionPointerBuilder<'a, Ump> {
    type Message = SongPositionPointerMessage<'a, Ump>;
    fn build(self) -> Result<SongPositionPointerMessage<'a, Ump>> {
        match self.0 {
            Ok(buffer) => Ok(SongPositionPointerMessage(buffer)),
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

impl<'a> Builder<'a, Bytes> for SongPositionPointerBuilder<'a, Bytes> {
    type Message = SongPositionPointerMessage<'a, Bytes>;
    fn new(buffer: &'a mut <Bytes as Buffer>::Data) -> Self {
        if buffer.len() >= 3 {
            message_helpers::clear_buffer(buffer);
            buffer[0] = OP_CODE;
            Self(Ok(buffer))
        } else {
            Self(Err(Error::BufferOverflow))
        }
    }
    fn build(self) -> Result<Self::Message> {
        match self.0 {
            Ok(buffer) => Ok(SongPositionPointerMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl<'a> GroupedBuilder<'a> for SongPositionPointerBuilder<'a, Ump> {
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
            SongPositionPointerMessage::<Ump>::builder(&mut Ump::random_buffer::<1>())
                .group(u4::new(0xA))
                .position(u14::new(0x367D))
                .build(),
            Ok(SongPositionPointerMessage::<Ump>(&[0x1AF2_7D6C])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            SongPositionPointerMessage::<Ump>::from_data(&[0x1AF2_7D6C])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn position() {
        assert_eq!(
            SongPositionPointerMessage::<Ump>::from_data(&[0x1AF2_7D6C])
                .unwrap()
                .position(),
            u14::new(0x367D),
        );
    }

    #[test]
    fn bytes_builder() {
        assert_eq!(
            SongPositionPointerMessage::<Bytes>::builder(&mut Bytes::random_buffer::<3>())
                .position(u14::new(0x367D))
                .build(),
            Ok(SongPositionPointerMessage::<Bytes>(&[0xF2, 0x7D, 0x6C])),
        );
    }

    #[test]
    fn bytes_position() {
        assert_eq!(
            SongPositionPointerMessage::<Bytes>::from_data(&[0xF2, 0x7D, 0x6C])
                .unwrap()
                .position(),
            u14::new(0x367D),
        );
    }
}
