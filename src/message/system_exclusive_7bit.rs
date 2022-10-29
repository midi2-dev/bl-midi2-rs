use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
    util::{SliceData, Truncate},
};

#[derive(
    Clone,
    Debug,
    PartialEq, Eq,
)]
pub struct Message {
    group: ux::u4,
    status: Status,
    data: Data,
}

pub type Data = SliceData<ux::u7, 6>;

impl Message {
    const TYPE_CODE: ux::u4 = ux::u4::new(0x3);
}

#[derive(
    Copy, 
    Clone, 
    Debug, 
    PartialEq, Eq,
)]
pub enum Status {
    Complete,
    Begin,
    Continue,
    End,
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        validate_type(&p)?;
        Ok(Message {
            group: super::helpers::group_from_packet(&p),
            status: status_from_packet(&p)?,
            data: data_from_packet(&p)?,
        })
    }
}

fn validate_type(p: &Packet) -> Result<(), Error> {
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
        0x3 => Ok(Status::End),
        _ => Err(Error::InvalidData),
    }
}

fn data_from_packet(p: &Packet) -> Result<Data, Error> {
    let n: usize = u8::from(p.nibble(3)).into();
    if n > 6 {
        Err(Error::InvalidData)
    } else {
        let mut data = SliceData::default();
        data.resize(n);
        for i in 0..n {
            data[i] = ux::u7::new(p.octet(2 + i));
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
        });
        let n: ux::u4 = u8::try_from(m.data.len()).unwrap().truncate();
        p.set_nibble(3, n);
        for (i, d) in m.data.iter().enumerate() {
            p.set_octet(i + 2, (*d).into());
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_message_type() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x2000_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x30A0_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x3009_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x3003_1234, 
                0x5600_0000,
            ])),
            Ok(Message {
                group: ux::u4::new(0x0),
                status: Status::Complete,
                data: Data::from_data(&[
                    ux::u7::new(0x12), 
                    ux::u7::new(0x34), 
                    ux::u7::new(0x56),
                ]),
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x4),
                status: Status::End,
                data: Data::from_data(&[
                    ux::u7::new(0x31),
                    ux::u7::new(0x41),
                    ux::u7::new(0x59),
                ]),
            }),
            Packet::from_data(&[0x3433_3141, 0x5900_0000])
        );
    }
}

// ux missing From usize impls:
//
// the ux crate does note imlement From<usize> on its types :-/
// Should be forthcoming in a future release.
