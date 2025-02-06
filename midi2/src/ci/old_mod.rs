use crate::{
    util::{Encode7Bit, Truncate},
    *,
};

pub use discovery::query::DiscoveryQuery;
pub use discovery::query::DiscoveryQueryBorrowed;
pub use discovery::query::DiscoveryQueryBorrowedBuilder;
pub use discovery::reply::DiscoveryReply;
pub use discovery::reply::DiscoveryReplyBorrowed;
pub use discovery::reply::DiscoveryReplyBorrowedBuilder;
pub use invalidate_muid::InvalidateMuid;
pub use invalidate_muid::InvalidateMuidBorrowed;
pub use invalidate_muid::InvalidateMuidBorrowedBuilder;
pub use nak::Nak;
pub use nak::NakBorrowed;
pub use nak::NakBorrowedBuilder;

mod discovery;
mod helpers;
mod invalidate_muid;
mod nak;

// todo: bump
const VERSION: u7 = u7::new(0x01);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceId {
    Channel(u4),
    Group,
    FunctionBlock,
}

impl core::default::Default for DeviceId {
    fn default() -> Self {
        Self::Channel(Default::default())
    }
}

impl DeviceId {
    pub(crate) fn from_u8(v: u8) -> Result<DeviceId> {
        if v == 0x7F {
            Ok(DeviceId::FunctionBlock)
        } else if v == 0x7E {
            Ok(DeviceId::Group)
        } else if v < 0x0F {
            Ok(DeviceId::Channel(v.try_into().unwrap()))
        } else {
            Err(Error::InvalidData)
        }
    }
    pub(crate) fn to_u8(self) -> u8 {
        match self {
            DeviceId::Group => 0x7E,
            DeviceId::FunctionBlock => 0x7F,
            DeviceId::Channel(c) => c.into(),
        }
    }
}

pub trait Ci: ByteData {
    fn device_id(&self) -> DeviceId {
        DeviceId::from_u8(self.byte_data()[2]).unwrap()
    }
    fn version(&self) -> u7 {
        self.byte_data()[5].truncate()
    }
    fn source(&self) -> u28 {
        u28::from_u7s(&self.byte_data()[6..10])
    }
    fn destination(&self) -> u28 {
        u28::from_u7s(&self.byte_data()[10..14])
    }
}

#[derive(Default)]
pub struct CiStandardData {
    pub device_id: DeviceId,
    pub source: Option<u28>,
    pub destination: Option<u28>,
    pub sysex_sub_id2: Option<u7>,
}

pub struct CiStandardDataIterator<'a>(&'a CiStandardData, [u7; 13], usize);

const UNIVERSAL_SYSEX: u7 = u7::new(0x7E);
const UNIVERSAL_SYSEX_SUB_ID_MIDI_CI: u7 = u7::new(0x0D);

impl CiStandardData {
    fn payload(&self) -> Result<CiStandardDataIterator> {
        let Some(source) = self.source else {
            return Err(Error::InvalidData);
        };
        let Some(destination) = self.destination else {
            return Err(Error::InvalidData);
        };
        let Some(sysex_sub_id2) = self.sysex_sub_id2 else {
            return Err(Error::InvalidData);
        };

        let mut data = [u7::default(); 13];
        data[0] = UNIVERSAL_SYSEX;
        data[1] = self.device_id.to_u8().truncate();
        data[2] = UNIVERSAL_SYSEX_SUB_ID_MIDI_CI;
        data[3] = sysex_sub_id2;
        data[4] = VERSION;
        source.to_u7s(&mut data[5..9]);
        destination.to_u7s(&mut data[9..13]);

        Ok(CiStandardDataIterator(self, data, 0))
    }
}

impl<'a> core::iter::Iterator for CiStandardDataIterator<'a> {
    type Item = u7;
    fn next(&mut self) -> Option<Self::Item> {
        if self.2 == self.1.len() {
            None
        } else {
            let ret = Some(self.1[self.2]);
            self.2 += 1;
            ret
        }
    }
}
