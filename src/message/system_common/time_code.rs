use crate::{
    message::{
        helpers as message_helpers,
        system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
    },
    result::Result,
    util::{BitOps, Truncate},
    *,
};

const OP_CODE: u8 = 0xF1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeCodeMessage<'a, B: Buffer>(&'a B::Data);

impl<'a> TimeCodeMessage<'a, Ump> {
    pub fn time_code(&self) -> u7 {
        self.0[0].octet(2).truncate()
    }
}

impl<'a> Message<'a, Ump> for TimeCodeMessage<'a, Ump> {
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

impl<'a> Buildable<'a, Ump> for TimeCodeMessage<'a, Ump> {
    type Builder = TimeCodeBuilder<'a, Ump>;
}

impl<'a> Buildable<'a, Bytes> for TimeCodeMessage<'a, Bytes> {
    type Builder = TimeCodeBuilder<'a, Bytes>;
}

impl<'a> GroupedMessage<'a> for TimeCodeMessage<'a, Ump> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct TimeCodeBuilder<'a, B: Buffer>(Result<&'a mut B::Data>);

impl<'a> TimeCodeBuilder<'a, Ump> {
    pub fn time_code(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, v.into());
        }
        self
    }
}

impl<'a> Builder<'a, Ump> for TimeCodeBuilder<'a, Ump> {
    type Message = TimeCodeMessage<'a, Ump>;
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
    fn build(self) -> Result<Self::Message> {
        match self.0 {
            Ok(buffer) => Ok(TimeCodeMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl<'a> GroupedBuilder<'a> for TimeCodeBuilder<'a, Ump> {
    fn group(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(v, buffer);
        }
        self
    }
}

impl<'a> TimeCodeMessage<'a, Bytes> {
    pub fn time_code(&self) -> u7 {
        self.0[1].truncate()
    }
}

impl<'a> TimeCodeBuilder<'a, Bytes> {
    pub fn time_code(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            buffer[1] = v.into();
        }
        self
    }
}

impl<'a> Message<'a, Bytes> for TimeCodeMessage<'a, Bytes> {
    fn data(&self) -> &'a <Bytes as Buffer>::Data {
        self.0
    }
    fn from_data_unchecked(buffer: &'a <Bytes as Buffer>::Data) -> Self {
        Self(buffer)
    }
    fn validate_data(buffer: &'a <Bytes as Buffer>::Data) -> Result<()> {
        system_common::validate_bytes(buffer, OP_CODE, 2)?;
        Ok(())
    }
}

impl<'a> Builder<'a, Bytes> for TimeCodeBuilder<'a, Bytes> {
    type Message = TimeCodeMessage<'a, Bytes>;
    fn new(buffer: &'a mut [u8]) -> Self {
        if buffer.len() >= 2 {
            message_helpers::clear_buffer(buffer);
            buffer[0] = OP_CODE;
            Self(Ok(buffer))
        } else {
            Self(Err(Error::BufferOverflow))
        }
    }
    fn build(self) -> Result<Self::Message> {
        match self.0 {
            Ok(buffer) => Ok(TimeCodeMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            TimeCodeMessage::<Ump>::builder(&mut Ump::random_buffer::<1>())
                .group(u4::new(0x5))
                .time_code(u7::new(0x5F))
                .build(),
            Ok(TimeCodeMessage::<Ump>(&[0x15F1_5F00])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TimeCodeMessage::<Ump>::from_data(&[0x15F1_5F00])
                .unwrap()
                .group(),
            u4::new(0x5),
        );
    }

    #[test]
    fn time_code() {
        assert_eq!(
            TimeCodeMessage::<Ump>::from_data(&[0x15F1_5F00])
                .unwrap()
                .time_code(),
            u7::new(0x5F),
        );
    }

    #[test]
    fn bytes_builder() {
        assert_eq!(
            TimeCodeMessage::<Bytes>::builder(&mut Bytes::random_buffer::<3>())
                .time_code(u7::new(0x5F))
                .build(),
            Ok(TimeCodeMessage::<Bytes>(&[0xF1, 0x5F, 0x00])),
        );
    }

    #[test]
    fn bytes_time_code() {
        assert_eq!(
            TimeCodeMessage::<Bytes>::from_data(&[0xF1, 0x5F, 0x00])
                .unwrap()
                .time_code(),
            u7::new(0x5F),
        );
    }
}
