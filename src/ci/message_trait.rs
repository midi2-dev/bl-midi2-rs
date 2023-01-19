use crate::{
    error::Error,
    message::{
        system_exclusive_7bit::Message as Sysex7Message,
        system_exclusive_8bit::Message as Sysex8Message,
    },
    util::sysex_message::SysexMessage,
};

pub trait CiMessage: Sized + CiMessageDetail {
    fn to_sysex8<'a>(
        &self,
        messages: &'a mut [Sysex8Message],
        stream_id: u8,
    ) -> &'a [Sysex8Message];
    fn from_sysex8(messages: &[Sysex8Message]) -> Self;
    fn validate_sysex8(message: &[Sysex8Message]) -> Result<(), Error>;
    fn validate_to_sysex8_buffer(&self, messages: &[Sysex8Message]) -> Result<(), Error>;
    fn try_from_sysex8(messages: &[Sysex8Message]) -> Result<Self, Error> {
        <Self as CiMessage>::validate_sysex8(messages)?;
        Ok(<Self as CiMessage>::from_sysex8(messages))
    }
    fn try_to_sysex8<'a>(
        &self,
        messages: &'a mut [Sysex8Message],
        stream_id: u8,
    ) -> Result<&'a [Sysex8Message], Error> {
        self.validate_to_sysex8_buffer(messages)?;
        Ok(self.to_sysex8(messages, stream_id))
    }
    fn to_sysex7<'a>(&self, messages: &'a mut [Sysex7Message]) -> &'a [Sysex7Message];
    fn from_sysex7(messages: &[Sysex7Message]) -> Self;
    fn validate_sysex7(message: &[Sysex7Message]) -> Result<(), Error>;
    fn validate_to_sysex7_buffer(&self, messages: &[Sysex7Message]) -> Result<(), Error>;
    fn try_from_sysex7(messages: &[Sysex7Message]) -> Result<Self, Error> {
        <Self as CiMessage>::validate_sysex7(messages)?;
        Ok(<Self as CiMessage>::from_sysex7(messages))
    }
    fn try_to_sysex7<'a>(
        &self,
        messages: &'a mut [Sysex7Message],
    ) -> Result<&'a [Sysex7Message], Error> {
        self.validate_to_sysex7_buffer(messages)?;
        Ok(self.to_sysex7(messages))
    }
}

impl<M: CiMessageDetail> CiMessage for M {
    fn to_sysex8<'a>(
        &self,
        messages: &'a mut [Sysex8Message],
        stream_id: u8,
    ) -> &'a [Sysex8Message] {
        let ret = self.to_sysex(messages);
        super::helpers::write_stream_id(ret, stream_id);
        ret
    }
    fn from_sysex8(messages: &[Sysex8Message]) -> Self {
        <Self as CiMessageDetail>::from_sysex(messages)
    }
    fn validate_sysex8(message: &[Sysex8Message]) -> Result<(), Error> {
        <Self as CiMessageDetail>::validate_sysex(message)
    }
    fn validate_to_sysex8_buffer(&self, messages: &[Sysex8Message]) -> Result<(), Error> {
        <Self as CiMessageDetail>::validate_to_sysex_buffer(self, messages)
    }
    fn to_sysex7<'a>(&self, messages: &'a mut [Sysex7Message]) -> &'a [Sysex7Message] {
        self.to_sysex(messages)
    }
    fn from_sysex7(messages: &[Sysex7Message]) -> Self {
        <Self as CiMessageDetail>::from_sysex(messages)
    }
    fn validate_sysex7(message: &[Sysex7Message]) -> Result<(), Error> {
        <Self as CiMessageDetail>::validate_sysex(message)
    }
    fn validate_to_sysex7_buffer(&self, messages: &[Sysex7Message]) -> Result<(), Error> {
        <Self as CiMessageDetail>::validate_to_sysex_buffer(self, messages)
    }
}

pub trait CiMessageDetail {
    fn to_sysex<'a, M: SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M];
    fn from_sysex<M: SysexMessage>(messages: &[M]) -> Self;
    fn validate_sysex<M: SysexMessage>(messages: &[M]) -> Result<(), Error>;
    fn validate_to_sysex_buffer<M: SysexMessage>(&self, messages: &[M]) -> Result<(), Error>;
}
