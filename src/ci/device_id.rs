use ux::u4;

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
    pub(crate) fn from_u8(v: u8) -> Result<DeviceId, crate::error::InvalidData> {
        if v == 0x7F {
            Ok(DeviceId::FunctionBlock)
        } else if v == 0x7E {
            Ok(DeviceId::Group)
        } else if v < 0x0F {
            Ok(DeviceId::Channel(v.try_into().unwrap()))
        } else {
            Err(crate::error::InvalidData(
                "Couldn't interpreset Device ID field",
            ))
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
