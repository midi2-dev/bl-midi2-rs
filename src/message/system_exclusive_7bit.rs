use crate::{
    error::Error,
    message::{helpers, Midi2Message},
    util::{builder, getter, sysex_message, BitOps, SliceData, Truncate},
};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    status: Status,
    data: Data,
}

#[derive(Clone)]
pub struct Builder {
    group: Option<ux::u4>,
    status: Option<Status>,
    data: Data,
}

impl Builder {
    builder::builder_setter!(group: ux::u4);
    builder::builder_setter!(status: Status);

    pub fn try_data(&mut self, v: &[ux::u7]) -> Result<&mut Self, Error> {
        if v.len() > 6 {
            Err(Error::BufferOverflow)
        } else {
            Ok(self.data(v))
        }
    }

    pub fn data(&mut self, v: &[ux::u7]) -> &mut Self {
        self.data = Data::from_data(v);
        self
    }

    pub fn build(&self) -> Message {
        Message {
            group: self.group.unwrap_or_else(|| panic!("Missing fields!")),
            status: self.status.unwrap_or_else(|| panic!("Missing fields!")),
            data: self.data.clone(),
        }
    }
}

type Data = SliceData<ux::u7, 6>;

impl Message {
    const TYPE_CODE: ux::u4 = ux::u4::new(0x3);

    pub fn builder() -> Builder {
        Builder {
            group: None,
            status: None,
            data: Data::default(),
        }
    }

    getter::getter!(group, ux::u4);
    getter::getter!(status, Status);

    pub fn data(&self) -> &[ux::u7] {
        &self.data
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Complete,
    Begin,
    Continue,
    End,
}

impl core::default::Default for Status {
    fn default() -> Self {
        Status::Complete
    }
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        validate_data(bytes)?;
        validate_status(bytes)?;
        validate_type(bytes)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: super::helpers::group_from_packet(bytes),
            status: status_from_packet(bytes),
            data: data_from_packet(bytes),
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
            },
        );
        let n: ux::u4 = u8::try_from(self.data.len()).unwrap().truncate();
        bytes[0].set_nibble(3, n);
        for (d, i) in self.data.iter().zip(2_usize..) {
            bytes[i / 4].set_octet(i % 4, (*d).into());
        }
        &bytes[..2]
    }
}

fn validate_type(p: &[u32]) -> Result<(), Error> {
    if p[0].nibble(0) != Message::TYPE_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn status_from_packet(p: &[u32]) -> Status {
    match u8::from(p[0].nibble(2)) {
        0x0 => Status::Complete,
        0x1 => Status::Begin,
        0x2 => Status::Continue,
        0x3 => Status::End,
        _ => panic!("Invalid status"),
    }
}

fn validate_status(p: &[u32]) -> Result<(), Error> {
    match u8::from(p[0].nibble(2)) {
        0x0 => Ok(()),
        0x1 => Ok(()),
        0x2 => Ok(()),
        0x3 => Ok(()),
        _ => Err(Error::InvalidData),
    }
}

fn data_from_packet(p: &[u32]) -> Data {
    let n: usize = u8::from(p[0].nibble(3)).into();
    let mut data = SliceData::default();
    data.resize(n);
    for i in 0..n {
        data[i] = ux::u7::new(p[(i + 2) / 4].octet((2 + i) % 4));
    }
    data
}

fn validate_data(p: &[u32]) -> Result<(), Error> {
    let n: usize = u8::from(p[0].nibble(3)).into();
    if n > 6 {
        Err(Error::InvalidData)
    } else if n > 2 && p.len() < 2 {
        Err(Error::BufferOverflow)
    } else {
        Ok(())
    }
}

impl sysex_message::SysexMessage for Message {
    fn group(&self) -> ux::u4 {
        self.group
    }
    fn set_group(&mut self, group: ux::u4) {
        self.group = group;
    }
    fn datum(&self, i: usize) -> u8 {
        self.data[i].into()
    }
    fn set_datum(&mut self, d: u8, i: usize) {
        if i >= self.data.len() {
            self.data.resize(i + 1);
        }
        self.data[i] = d.truncate();
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn max_len() -> usize {
        6
    }
    fn status(&self) -> sysex_message::Status {
        match self.status {
            Status::Complete => sysex_message::Status::Complete,
            Status::Begin => sysex_message::Status::Begin,
            Status::Continue => sysex_message::Status::Continue,
            Status::End => sysex_message::Status::End,
        }
    }
    fn set_status(&mut self, status: sysex_message::Status) {
        match status {
            sysex_message::Status::Complete => self.status = Status::Complete,
            sysex_message::Status::Begin => self.status = Status::Begin,
            sysex_message::Status::Continue => self.status = Status::Continue,
            sysex_message::Status::End => self.status = Status::End,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_message_type() {
        assert_eq!(
            Message::try_from_ump(&[0x2000_0000]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Message::try_from_ump(&[0x30A0_0000]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Message::try_from_ump(&[0x3009_0000]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x3003_1234, 0x5600_0000,]),
            Ok(Message {
                group: ux::u4::new(0x0),
                status: Status::Complete,
                data: Data::from_data(&[ux::u7::new(0x12), ux::u7::new(0x34), ux::u7::new(0x56),]),
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x4),
                status: Status::End,
                data: Data::from_data(&[ux::u7::new(0x31), ux::u7::new(0x41), ux::u7::new(0x59),]),
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x3433_3141, 0x5900_0000],
        );
    }

    #[test]
    fn serialize_small_data() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x4),
                status: Status::End,
                data: Data::from_data(&[ux::u7::new(0x31)]),
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x3431_3100, 0x0],
        );
    }

    #[test]
    fn serialize_big_data() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x4),
                status: Status::Begin,
                data: Data::from_data(&[
                    ux::u7::new(0x31),
                    ux::u7::new(0x31),
                    ux::u7::new(0x31),
                    ux::u7::new(0x31),
                    ux::u7::new(0x31),
                    ux::u7::new(0x31),
                ]),
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x3416_3131, 0x3131_3131],
        );
    }
}
