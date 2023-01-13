use crate::{
    error::Error,
    ci::CiMessageDetail,
    util::{builder, getter, sysex_message},
};

pub struct Message {
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7,
}

impl Message {
    const STATUS: u8 = 0x15;
    getter::getter!(group, ux::u4);
    getter::getter!(source, ux::u28);
    getter::getter!(destination, ux::u28);
    getter::getter!(authority_level, ux::u7);
}

builder::builder!(
    group: ux::u4,
    source: ux::u28,
    destination: ux::u28,
    authority_level: ux::u7
);

impl CiMessageDetail for Message {
    fn to_sysex<'a, M: sysex_message::SysexMessage>(&self, messages: &'a mut [M]) -> &'a mut [M] {
        todo!()
    }

    fn from_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Self {
        todo!()
    }

    fn validate_sysex<M: sysex_message::SysexMessage>(messages: &[M]) -> Result<(), Error> {
        todo!()
    }

    fn validate_to_sysex_buffer<M: sysex_message::SysexMessage>(&self, messages: &[M]) -> Result<(), Error> {
        todo!()
    }
}