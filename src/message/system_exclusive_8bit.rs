use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
    util::{SliceData, Truncate},
};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message {
    group: ux::u4,
    stream_id: u8,
    status: Status,
    data: Data,
}

pub type Data = SliceData<u8, 13>;

impl Message {
    const TYPE_CODE: ux::u4 = ux::u4::new(0x5);
}

#[derive(
    Copy, 
    Clone, 
    Debug, 
    PartialEq,
)]
pub enum Status {
    Complete,
    Begin,
    Continue,
    End,
    UnexpectedEnd(Validity),
}

#[derive(
    Copy, 
    Clone, 
    Debug, 
    PartialEq,
)]
pub enum Validity {
    Valid,
    Invalid,
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        validate_packet(&p)?;
        let status = status_from_packet(&p)?;
        Ok(Message {
            group: super::helpers::group_from_packet(&p),
            stream_id: p.octet(2),
            data: data_from_packet(&p, status)?,
            status,
        })
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error> {
    if p.nibble(0) != Message::TYPE_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())        
    }
}

fn status_from_packet(p: &Packet) -> Result<Status, Error> {
    match u8::from(p.nibble(2)) {
        0x0 => Ok(Status::Complete),
        0x1 => Ok(Status::Begin),
        0x2 => Ok(Status::Continue),
        0x3 => {
            let all_data_set_to_zero = 
                p[1..3].iter().all(|b| *b == 0) 
                && p[0] & 0x0000_0011 == 0x0;
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

fn number_of_bytes(p: &Packet) -> ux::u4 {
    p.nibble(3)
}

fn data_from_packet(p: &Packet, status: Status) -> Result<Data, Error> {
    let n: usize = u8::from(number_of_bytes(p)).into();
    let unexpected_end = match status {
        Status::UnexpectedEnd(_) => true,
        _ => false,
    };
    if n == 0 {
        // we expect a stream id
        Err(Error::InvalidData)
    } else if unexpected_end {
        Ok(Data::default())
    } else if n > 14 {
        Err(Error::InvalidData)
    } else {
        let mut data = SliceData::default();
        data.resize(n - 1);
        for i in 1..n {
            data[i - 1] = p.octet(2 + i);
        }
        Ok(data)
    }
}

impl core::convert::From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        super::write_type_to_packet(Message::TYPE_CODE, &mut p);
        p.set_nibble(1, m.group);
        p.set_nibble(2, match m.status {
            Status::Complete => ux::u4::new(0x0),
            Status::Begin => ux::u4::new(0x1),
            Status::Continue => ux::u4::new(0x2),
            Status::End => ux::u4::new(0x3),
            Status::UnexpectedEnd(_) => ux::u4::new(0x3),
        });
        p.set_octet(2, m.stream_id);
        let n: ux::u4 = u8::try_from(m.data.len()).unwrap().truncate();
        p.set_nibble(3, n + ux::u4::new(1));
        for (i, d) in m.data.iter().enumerate() {
            p.set_octet(i + 3, *d);
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x5B0C_AB01, 
                0x0203_0405,
                0x0607_0809,
                0x1011_0000,
            ])),
            Ok(Message {
                group: ux::u4::new(0xB),
                status: Status::Complete,
                stream_id: 0xAB,
                data: Data::from_data(&[
                    0x01,
                    0x02,
                    0x03,
                    0x04,
                    0x05,
                    0x06,
                    0x07,
                    0x08,
                    0x09,
                    0x10,
                    0x11,
                ]),
            }),
        );
    }

    #[test]
    fn deserialize_unexpected_end_valid() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x5731_2100])),
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
            Message::try_from(Packet::from_data(&[0x533F_B000])),
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
            Packet::from(Message {
                group: ux::u4::new(0xF),
                status: Status::End,
                stream_id: 0xDA,
                data: Data::from_data(&[
                    0x01,
                    0x02,
                    0x03,
                    0x04,
                    0x05,
                    0x06,
                    0x07,
                    0x08,
                    0x09,
                    0x0A,
                ]),
            }),
            Packet::from_data(&[
                0x5F3B_DA01, 
                0x0203_0405, 
                0x0607_0809, 
                0x0A00_0000
            ]),
        );
    }
}