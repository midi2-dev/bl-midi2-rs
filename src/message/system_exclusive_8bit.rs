use crate::{
    error::Error,
    result::Result,
    message::helpers as message_helpers,
    util::{BitOps, Truncate, debug},
};

pub struct PayloadIterator<'a> {
    data: &'a [u32],
    index: usize,
    total: usize,
}

impl<'a> core::iter::Iterator for PayloadIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.total {
            None
        } else {
            let ret = self.data[(self.index + 3) / 4].octet((self.index + 3) % 4);
            self.index += 1;
            Some(ret)
        }
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
            total: u32::from(self.0[0].nibble(3) - ux::u4::new(1)) as usize,
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
            // start at one because we always have
            // a stream id
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
    if buf.len() < 4 {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
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
    
    #[test]
    fn builder_large_payload() {
        assert_eq!(
            Sysex8Message::builder(&mut [0x0; 4])
                .payload([0x0; 14].iter())
                .build(),
            Err(Error::InvalidData),
        )
    }
    
    #[test]
    fn must_have_stream_id() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5000_0000, 0x0, 0x0, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5C01_0000, 0x0, 0x0, 0x0]).unwrap().group(),
            ux::u4::new(0xC),
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5001_9900, 0x0, 0x0, 0x0]).unwrap().stream_id(),
            0x99,
        );
    }

    #[test]
    fn status() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5021_0000, 0x0, 0x0, 0x0]).unwrap().status(),
            Status::Continue,
        );
    }

    #[test]
    fn status_end() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5032_0000, 0x0, 0x0, 0x0]).unwrap().status(),
            Status::End,
        );
    }

    #[test]
    fn status_unexpected_end_valid() {
        assert_eq!(
            Sysex8Message::from_data(&[0x5031_0000, 0x0, 0x0, 0x0]).unwrap().status(),
            Status::UnexpectedEnd(Validity::Valid),
        );
    }

    #[test]
    fn status_unexpected_end_invalid() {
        assert_eq!(
            Sysex8Message::from_data(&[0x503F_0000, 0x0, 0x0, 0x0]).unwrap().status(),
            Status::UnexpectedEnd(Validity::Invalid),
        );
    }
    
    #[test]
    fn payload() {
        let message = Sysex8Message::from_data(&[0x5009_FF00, 0x1122_3344, 0x5566_7700, 0x0]).unwrap();
        let mut buffer = [0u8; 8];
        for (i, v) in message.payload().enumerate() {
            buffer[i] = v;
        }
        assert_eq!(&buffer, &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]);
    }
}
