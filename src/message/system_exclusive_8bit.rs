use crate::{
    error::Error,
    message::{helpers, Midi2Message},
    util::{builder, getter, BitOps, SliceData, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    stream_id: u8,
    status: Status,
    data: Data,
}

#[derive(Clone)]
pub struct Builder {
    group: Option<ux::u4>,
    stream_id: Option<u8>,
    status: Option<Status>,
    data: Data,
}

impl Builder {
    builder::builder_setter!(group: ux::u4);
    builder::builder_setter!(stream_id: u8);

    pub fn status(&mut self, status: Status) -> &mut Self {
        if let Status::UnexpectedEnd(_) = status {
            self.data.resize(0);
        }
        self.status = Some(status);
        self
    }

    pub fn try_data(&mut self, v: &[u8]) -> Result<&mut Self, Error> {
        if v.len() > 6 {
            Err(Error::BufferOverflow)
        } else {
            Ok(self.data(v))
        }
    }

    pub fn data(&mut self, v: &[u8]) -> &mut Self {
        self.data = Data::from_data(v);
        self
    }

    pub fn build(&self) -> Message {
        Message {
            group: self.group.unwrap_or_else(|| panic!("Missing fields!")),
            stream_id: self.stream_id.unwrap_or_else(|| panic!("Missing fields!")),
            status: self.status.unwrap_or_else(|| panic!("Missing fields!")),
            data: self.data.clone(),
        }
    }
}

type Data = SliceData<u8, 13>;

impl Message {
    const TYPE_CODE: ux::u4 = ux::u4::new(0x5);
    getter::getter!(group, ux::u4);
    getter::getter!(stream_id, u8);
    getter::getter!(status, Status);

    pub fn data(&self) -> &[u8] {
        &*self.data
    }

    pub fn builder() -> Builder {
        Builder {
            group: None,
            stream_id: None,
            status: None,
            data: Data::default(),
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Validity {
    Valid,
    Invalid,
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        match try_status_from_packet(bytes) {
            Ok(status) => {
                validate_data(bytes, status)?;
                validate_packet(bytes)
            }
            Err(e) => Err(e),
        }
    }
    fn from_ump(bytes: &[u32]) -> Self {
        let status = try_status_from_packet(bytes).expect("Valid status");
        Message {
            group: super::helpers::group_from_packet(bytes),
            stream_id: bytes[0].octet(2),
            data: data_from_packet(bytes, status),
            status,
        }
    }
    fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32] {
        helpers::write_type_to_packet(Message::TYPE_CODE, bytes);
        bytes[0].set_nibble(1, self.group);
        bytes[0].set_nibble(
            2,
            match self.status {
                Status::Complete => ux::u4::new(0x0),
                Status::Begin => ux::u4::new(0x1),
                Status::Continue => ux::u4::new(0x2),
                Status::End => ux::u4::new(0x3),
                Status::UnexpectedEnd(_) => ux::u4::new(0x3),
            },
        );
        if let Status::UnexpectedEnd(validity) = self.status {
            bytes[0] &= 0xFFFF_FF00;
            bytes[1..4].copy_from_slice(&[0x0, 0x0, 0x0]);
            match validity {
                Validity::Valid => {
                    bytes[0].set_nibble(3, ux::u4::new(0x1));
                }
                Validity::Invalid => {
                    bytes[0].set_nibble(3, ux::u4::new(0xF));
                }
            }
        } else {
            bytes[0].set_octet(2, self.stream_id);
            let n: ux::u4 = u8::try_from(self.data.len()).unwrap().truncate();
            bytes[0].set_nibble(3, n + ux::u4::new(1));
            for (d, i) in self.data.iter().zip(3_usize..) {
                bytes[i / 4].set_octet(i % 4, *d);
            }
        }
        &bytes[..4]
    }
}

fn validate_packet(p: &[u32]) -> Result<(), Error> {
    if p[0].nibble(0) != Message::TYPE_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn try_status_from_packet(p: &[u32]) -> Result<Status, Error> {
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

fn data_from_packet(p: &[u32], status: Status) -> Data {
    let n: usize = u8::from(number_of_bytes(p)).into();
    let unexpected_end = matches!(status, Status::UnexpectedEnd(_));
    if unexpected_end {
        Data::default()
    } else {
        let mut data = SliceData::default();
        data.resize(n - 1);
        for i in 1..n {
            data[i - 1] = p[(i + 2) / 4].octet((2 + i) % 4);
        }
        data
    }
}

fn validate_data(p: &[u32], status: Status) -> Result<(), Error> {
    let n: usize = u8::from(number_of_bytes(p)).into();
    let unexpected_end = matches!(status, Status::UnexpectedEnd(_));
    if n == 0 {
        // we expect a stream id
        Err(Error::InvalidData)
    } else if unexpected_end {
        Ok(())
    } else if n > 14 {
        Err(Error::InvalidData)
    } else if (n + 3) / 4 > p.len() {
        Err(Error::BufferOverflow)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
