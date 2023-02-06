use crate::{
    error::Error,
    result::Result,
    message::helpers as message_helpers,
    util::{BitOps, Truncate, debug},
};

pub struct PayloadIterator<'a> {
    data: &'a [u32],
    index: u8,
    total: u8,
}

impl<'a> core::iter::Iterator for PayloadIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Sysex8Message<'a>(&'a [u32]);

impl<'a> Sysex8Message<'a> {
    const OP_CODE: ux::u4 = ux::u4::new(0x5);
    pub fn builder(buffer: &'a mut [u32]) -> Sysex8MessageBuilder<'a> {
        Sysex8MessageBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn status(&self) -> Status {
        try_status_from_packet(self.0).expect("Valid status")
    }
    pub fn stream_id(&self) -> u8 {
        self.0[0].octet(2)
    }
    pub fn payload(&self) -> PayloadIterator {
        PayloadIterator{
            data: self.0,
            index: 0,
            total: self.0[0].nibble(3).into(),
        }
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        validate_buffer(data)?;
        match try_status_from_packet(data) {
            Ok(status) => {
                validate_data(data, status)?;
                validate_packet(data)?;
                Ok(Sysex8Message(&data[..4]))
            }
            Err(e) => Err(e),
        }
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
}

debug::message_debug_impl!(Sysex8Message);

enum BuilderImpl<'a> {
    Ok(&'a mut [u32]),
    Err(Error),
}

pub struct Sysex8MessageBuilder<'a>(BuilderImpl<'a>);

impl<'a> Sysex8MessageBuilder<'a> {
    pub fn group(&mut self, g: ux::u4) -> &mut Self {
        if let BuilderImpl::Ok(buffer) = &mut self.0 {
            buffer[0].set_nibble(1, g);
        }
        self
    }
    /// When called with `Status::UnexpectedEnd(_)` the payload buffer
    /// will be filled with zeros accordingly.
    pub fn status(&mut self, s: Status) -> &mut Self {
        if let BuilderImpl::Ok(buffer) = &mut self.0 {
            buffer[0].set_nibble(
                2,
                match s {
                    Status::Complete => ux::u4::new(0x0),
                    Status::Begin => ux::u4::new(0x1),
                    Status::Continue => ux::u4::new(0x2),
                    Status::End => ux::u4::new(0x3),
                    Status::UnexpectedEnd(_) => ux::u4::new(0x3),
                },
            );
            if let Status::UnexpectedEnd(validity) = s {
                buffer[0] &= 0xFFFF_FF00;
                buffer[1..4].copy_from_slice(&[0x0, 0x0, 0x0]);
                match validity {
                    Validity::Valid => {
                        buffer[0].set_nibble(3, ux::u4::new(0x1));
                    }
                    Validity::Invalid => {
                        buffer[0].set_nibble(3, ux::u4::new(0xF));
                    }
                }
            }
        }
        self
    }
    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        if let BuilderImpl::Ok(buffer) = &mut self.0 {
            buffer[0].set_octet(2, id);
        }
        self
    }
    pub fn payload<'b, I: core::iter::Iterator<Item = &'b u8>>(&mut self, mut data: I) -> &mut Self {
        if let BuilderImpl::Ok(buffer) = &mut self.0 {
            let mut count = 1_u8;
            for i in 0_usize..13_usize {
                if let Some(&v) = data.next() {
                    buffer[(i + 3) / 4].set_octet((i + 3) % 4, v);
                    count += 1;
                } else {
                    break;
                }
            }
            if data.next().is_some() {
                self.0 = BuilderImpl::Err(Error::InvalidData);
            } else {
                buffer[0].set_nibble(3, count.truncate());
            }
        }
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        if buffer.len() >= 4 {
            let buffer = &mut buffer[..4];
            for v in buffer.iter_mut() {
                *v = 0;
            }
            message_helpers::write_type_to_packet(Sysex8Message::OP_CODE, buffer);
            buffer[0].set_nibble(3, ux::u4::new(0x1)); // stream id
            Self(BuilderImpl::Ok(buffer))
        } else {
            Self(BuilderImpl::Err(Error::BufferOverflow))
        }
    }
    pub fn build(&'a self) -> Result<Sysex8Message<'a>> {
        match &self.0 {
            BuilderImpl::Ok(buffer) => Ok(Sysex8Message(buffer)),
            BuilderImpl::Err(e) => Err(e.clone()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Complete,
    Begin,
    Continue,
    End,
    UnexpectedEnd(Validity),
}

impl core::default::Default for Status {
    fn default() -> Self {
        Status::Complete
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Validity {
    Valid,
    Invalid,
}

fn validate_packet(p: &[u32]) -> Result<()> {
    if p[0].nibble(0) != Sysex8Message::OP_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn validate_buffer(buf: &[u32]) -> Result<()> {
    todo!()
}

fn try_status_from_packet(p: &[u32]) -> Result<Status> {
    match u8::from(p[0].nibble(2)) {
        0x0 => Ok(Status::Complete),
        0x1 => Ok(Status::Begin),
        0x2 => Ok(Status::Continue),
        0x3 => {
            let all_data_set_to_zero = {
                if p[0] & 0x0000_0011 != 0x0 {
                    false
                } else if p.len() > 1 {
                    p[1..].iter().all(|b| *b == 0)
                } else {
                    true
                }
            };
            if all_data_set_to_zero {
                if number_of_bytes(p) == ux::u4::new(0x1) {
                    Ok(Status::UnexpectedEnd(Validity::Valid))
                } else if number_of_bytes(p) == ux::u4::new(0xF) {
                    Ok(Status::UnexpectedEnd(Validity::Invalid))
                } else {
                    Ok(Status::End)
                }
            } else {
                Ok(Status::End)
            }
        }
        _ => Err(Error::InvalidData),
    }
}

fn number_of_bytes(p: &[u32]) -> ux::u4 {
    p[0].nibble(3)
}

fn validate_data(p: &[u32], status: Status) -> Result<()> {
    let n: usize = u8::from(number_of_bytes(p)).into();
    let unexpected_end = matches!(status, Status::UnexpectedEnd(_));
    if n == 0 {
        // we expect a stream id
        Err(Error::InvalidData)
    } else if unexpected_end {
        // data should be set to zero
        // but we wont make it a hard requirement here
        Ok(())
    } else if n > 14 {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn builder() {
        assert_eq!(
            Sysex8Message::builder(&mut [0x0; 4])
                .group(ux::u4::new(0xA))
                .stream_id(0xC6)
                .status(Status::Continue)
                .payload([0x12, 0x34, 0x56, 0x78, 0x90].iter())
                .build(),
            Ok(Sysex8Message(&[0x5A26_C612, 0x3456_7890, 0x0, 0x0])),
        )
    }

    /*
    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x5B0C_AB01, 0x0203_0405, 0x0607_0809, 0x1011_0000,]),
            Ok(Message {
                group: ux::u4::new(0xB),
                status: Status::Complete,
                stream_id: 0xAB,
                data: Data::from_data(&[
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11,
                ]),
            }),
        );
    }

    #[test]
    fn deserialize_unexpected_end_valid() {
        assert_eq!(
            Message::try_from_ump(&[0x5731_2100]),
            Ok(Message {
                group: ux::u4::new(0x7),
                status: Status::UnexpectedEnd(Validity::Valid),
                stream_id: 0x21,
                data: Data::default(),
            }),
        );
    }

    #[test]
    fn deserialize_unexpected_end_invalid() {
        assert_eq!(
            Message::try_from_ump(&[0x533F_B000]),
            Ok(Message {
                group: ux::u4::new(0x3),
                status: Status::UnexpectedEnd(Validity::Invalid),
                stream_id: 0xB0,
                data: Data::default(),
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0xF),
                status: Status::End,
                stream_id: 0xDA,
                data: Data::from_data(&[
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
                ]),
            }
            .to_ump(&mut [0x0, 0x0, 0x0, 0x0]),
            &[0x5F3B_DA01, 0x0203_0405, 0x0607_0809, 0x0A00_0000],
        );
    }

    #[test]
    fn serialize_unexpected_end_invalid() {
        assert_eq!(
            Message {
                group: ux::u4::new(0xF),
                status: Status::UnexpectedEnd(Validity::Invalid),
                stream_id: 0xDA,
                data: Default::default(),
            }
            .to_ump(&mut [0x0, 0x0, 0x0, 0x0]),
            &[0x5F3F_0000, 0x0, 0x0, 0x0],
        );
    }
    */
}
