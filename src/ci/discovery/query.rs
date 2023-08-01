use crate::{
    result::Result,
    error::Error,
    message::{
        sysex,
        system_exclusive_8bit as sysex8,
        system_exclusive_7bit as sysex7,
    },
    util::{Encode7Bit, Truncate, BitOps},
    ci::{helpers as ci_helpers, DeviceId},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiscoveryQueryMessage<Repr: sysex::SysexMessages>(Repr);

const STATUS: u8 = 0x70;

#[repr(usize)]
enum DataOffsets {
    DeviceManufacturer = ci_helpers::STANDARD_DATA_SIZE,
    DeviceFamily = DataOffsets::DeviceManufacturer as usize + 3,
    DeviceFamilyModelNumber = DataOffsets::DeviceFamily as usize + 2,
    SoftwareVersion = DataOffsets::DeviceFamilyModelNumber as usize + 2,
    CiSupportFlags = DataOffsets::SoftwareVersion as usize + 4,
    MaxSysexSize = DataOffsets::CiSupportFlags as usize + 1,
}

impl<'a> DiscoveryQueryMessage<sysex8::Sysex8MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> DiscoveryQueryBuilder<sysex8::Sysex8MessageGroup<'a>> {
        DiscoveryQueryBuilder::<sysex8::Sysex8MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        self.0.group()
    }
    pub fn source(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        ux::u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn device_manufacturer(&self) -> ux::u21 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceManufacturer as usize - 1);
        ux::u21::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn device_family(&self) -> ux::u14 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceFamily as usize - 1);
        ux::u14::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn device_model_number(&self) -> ux::u14 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceFamilyModelNumber as usize - 1);
        ux::u14::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn software_version(&self) -> [ux::u7; 4] {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::SoftwareVersion as usize - 1);
        [
            payload.next().unwrap().truncate(),
            payload.next().unwrap().truncate(),
            payload.next().unwrap().truncate(),
            payload.next().unwrap().truncate(),
        ]
    }
    pub fn protocol_negotiation_supported(&self) -> bool {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::CiSupportFlags as usize).unwrap().bit(6)
    }
    pub fn profile_configuration_supported(&self) -> bool {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::CiSupportFlags as usize).unwrap().bit(5)
    }
    pub fn property_exchange_supported(&self) -> bool {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::CiSupportFlags as usize).unwrap().bit(4)
    }
    pub fn max_sysex_message_size(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::MaxSysexSize as usize - 1);
        ux::u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex8(data, STATUS)?;
        let mut payload = messages.payload();
        let Some(_) = payload.nth(DataOffsets::MaxSysexSize as usize) else {
            return Err(Error::InvalidData);
        };
        Ok(DiscoveryQueryMessage(messages))
    }
}

impl<'a> DiscoveryQueryMessage<sysex7::Sysex7MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> DiscoveryQueryBuilder<sysex7::Sysex7MessageGroup<'a>> {
        DiscoveryQueryBuilder::<sysex7::Sysex7MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        self.0.group()
    }
    pub fn source(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        ux::u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn device_manufacturer(&self) -> ux::u21 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceManufacturer as usize - 1);
        ux::u21::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn device_family(&self) -> ux::u14 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceFamily as usize - 1);
        ux::u14::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn device_model_number(&self) -> ux::u14 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceFamilyModelNumber as usize - 1);
        ux::u14::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn software_version(&self) -> [ux::u7; 4] {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::SoftwareVersion as usize - 1);
        [
            payload.next().unwrap().truncate(),
            payload.next().unwrap().truncate(),
            payload.next().unwrap().truncate(),
            payload.next().unwrap().truncate(),
        ]
    }
    fn flags_bit(&self) -> u8 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::CiSupportFlags as usize).unwrap().into()
    }
    pub fn protocol_negotiation_supported(&self) -> bool {
        self.flags_bit().bit(6)
    }
    pub fn profile_configuration_supported(&self) -> bool {
        self.flags_bit().bit(5)
    }
    pub fn property_exchange_supported(&self) -> bool {
        self.flags_bit().bit(4)
    }
    pub fn max_sysex_message_size(&self) -> ux::u28 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::MaxSysexSize as usize - 1);
        ux::u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        let messages = ci_helpers::validate_sysex7(data, STATUS)?;
        let mut payload = messages.payload();
        let Some(_) = payload.nth(DataOffsets::MaxSysexSize as usize) else {
            return Err(Error::InvalidData);
        };
        Ok(DiscoveryQueryMessage(messages))
    }
}

pub struct DiscoveryQueryBuilder<Repr: sysex::SysexMessages> {
    source: ux::u28,
    device_manufacturer: ux::u21,
    device_family: ux::u14,
    device_model_number: ux::u14,
    software_version: [ux::u7; 4],
    protocol_negotiation_supported: bool,
    profile_configuration_supported: bool,
    property_exchange_supported: bool,
    max_sysex_message_size: ux::u28,
    builder: Repr::Builder,
}

impl<'a> DiscoveryQueryBuilder<sysex8::Sysex8MessageGroup<'a>> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Self {
            builder: sysex8::Sysex8MessageGroupBuilder::new(buffer),
            source: Default::default(),
            device_manufacturer: Default::default(),
            device_family: Default::default(),
            device_model_number: Default::default(),
            software_version: Default::default(),
            protocol_negotiation_supported: false,
            profile_configuration_supported: false,
            property_exchange_supported: false,
            max_sysex_message_size: Default::default(),
        }
    }
    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        self.builder.stream_id(id);
        self
    }
    pub fn group(&mut self, group: ux::u4) -> &mut Self {
        self.builder.group(group);
        self
    }
    pub fn source(&mut self, source: ux::u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn device_manufacturer(&mut self, device_manufacturer: ux::u21) -> &mut Self {
        self.device_manufacturer = device_manufacturer;
        self
    }
    pub fn device_family(&mut self, device_family: ux::u14) -> &mut Self {
        self.device_family = device_family;
        self
    }
    pub fn device_model_number(&mut self, device_model_number: ux::u14) -> &mut Self {
        self.device_model_number = device_model_number;
        self
    }
    pub fn software_version(&mut self, software_version: [ux::u7; 4]) -> &mut Self {
        self.software_version = software_version;
        self
    }
    pub fn protocol_negotiation_supported(&mut self, protocol_negotiation_supported: bool) -> &mut Self {
        self.protocol_negotiation_supported = protocol_negotiation_supported;
        self
    }
    pub fn profile_configuration_supported(&mut self, profile_configuration_supported: bool) -> &mut Self {
        self.profile_configuration_supported = profile_configuration_supported;
        self
    }
    pub fn property_exchange_supported(&mut self, property_exchange_supported: bool) -> &mut Self {
        self.property_exchange_supported = property_exchange_supported;
        self
    }
    pub fn max_sysex_message_size(&mut self, max_sysex_message_size: ux::u28) -> &mut Self {
        self.max_sysex_message_size = max_sysex_message_size;
        self
    }
    pub fn build(&'a mut self) -> Result<DiscoveryQueryMessage<sysex8::Sysex8MessageGroup<'a>>> {
        let payload = ci_helpers::StandardDataIterator::new(
            DeviceId::MidiPort,
            STATUS,
            self.source,
            ux::u28::max_value(),
        );

        let device_manufacturer_array = self.device_manufacturer.to_u7s();
        let payload = payload.chain(device_manufacturer_array.iter().cloned().map(u8::from));

        let device_family_array = self.device_family.to_u7s();
        let payload = payload.chain(device_family_array.iter().cloned().map(u8::from));

        let device_model_number_array = self.device_model_number.to_u7s();
        let payload = payload.chain(device_model_number_array.iter().cloned().map(u8::from));

        let payload = payload.chain(self.software_version.iter().cloned().map(u8::from));

        let support_flags = {
            let mut bits: u8 = 0x0;
            bits.set_bit(6, self.protocol_negotiation_supported);
            bits.set_bit(5, self.profile_configuration_supported);
            bits.set_bit(4, self.property_exchange_supported);
            bits
        };
        let payload = payload.chain(core::iter::once(support_flags));

        let max_sysex_size_array = self.max_sysex_message_size.to_u7s();
        let payload = payload.chain(max_sysex_size_array.iter().cloned().map(u8::from));

        match self.builder.payload(payload).build() {
            Ok(messages) => Ok(DiscoveryQueryMessage(messages)),
            Err(e) => Err(e)
        }            
    }
}

impl<'a> DiscoveryQueryBuilder<sysex7::Sysex7MessageGroup<'a>> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Self {
            builder: sysex7::Sysex7MessageGroupBuilder::new(buffer),
            source: Default::default(),
            device_manufacturer: Default::default(),
            device_family: Default::default(),
            device_model_number: Default::default(),
            software_version: Default::default(),
            protocol_negotiation_supported: false,
            profile_configuration_supported: false,
            property_exchange_supported: false,
            max_sysex_message_size: Default::default(),
        }
    }
    pub fn group(&mut self, group: ux::u4) -> &mut Self {
        self.builder.group(group);
        self
    }
    pub fn source(&mut self, source: ux::u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn device_manufacturer(&mut self, device_manufacturer: ux::u21) -> &mut Self {
        self.device_manufacturer = device_manufacturer;
        self
    }
    pub fn device_family(&mut self, device_family: ux::u14) -> &mut Self {
        self.device_family = device_family;
        self
    }
    pub fn device_model_number(&mut self, device_model_number: ux::u14) -> &mut Self {
        self.device_model_number = device_model_number;
        self
    }
    pub fn software_version(&mut self, software_version: [ux::u7; 4]) -> &mut Self {
        self.software_version = software_version;
        self
    }
    pub fn protocol_negotiation_supported(&mut self, protocol_negotiation_supported: bool) -> &mut Self {
        self.protocol_negotiation_supported = protocol_negotiation_supported;
        self
    }
    pub fn profile_configuration_supported(&mut self, profile_configuration_supported: bool) -> &mut Self {
        self.profile_configuration_supported = profile_configuration_supported;
        self
    }
    pub fn property_exchange_supported(&mut self, property_exchange_supported: bool) -> &mut Self {
        self.property_exchange_supported = property_exchange_supported;
        self
    }
    pub fn max_sysex_message_size(&mut self, max_sysex_message_size: ux::u28) -> &mut Self {
        self.max_sysex_message_size = max_sysex_message_size;
        self
    }
    pub fn build(&'a mut self) -> Result<DiscoveryQueryMessage<sysex7::Sysex7MessageGroup<'a>>> {
        let payload = ci_helpers::StandardDataIterator::new(
            DeviceId::MidiPort,
            STATUS,
            self.source,
            ux::u28::max_value(),
        ).map(|v| v.truncate());

        let device_manufacturer_array = self.device_manufacturer.to_u7s();
        let payload = payload.chain(device_manufacturer_array.iter().cloned());

        let device_family_array = self.device_family.to_u7s();
        let payload = payload.chain(device_family_array.iter().cloned());

        let device_model_number_array = self.device_model_number.to_u7s();
        let payload = payload.chain(device_model_number_array.iter().cloned());

        let payload = payload.chain(self.software_version.iter().cloned());

        let support_flags = {
            let mut bits: u8 = 0x0;
            bits.set_bit(6, self.protocol_negotiation_supported);
            bits.set_bit(5, self.profile_configuration_supported);
            bits.set_bit(4, self.property_exchange_supported);
            bits
        };
        let payload = payload.chain(core::iter::once(support_flags.truncate()));

        let max_sysex_size_array = self.max_sysex_message_size.to_u7s();
        let payload = payload.chain(max_sysex_size_array.iter().cloned());

        match self.builder.payload(payload).build() {
            Ok(messages) => Ok(DiscoveryQueryMessage(messages)),
            Err(e) => Err(e)
        }            
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::debug;

    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::builder(&mut [0x0; 12])
                .group(ux::u4::new(0x8))
                .stream_id(0x31)
                .source(ux::u28::new(196099328))
                .device_manufacturer(ux::u21::new(2054957))
                .device_family(ux::u14::new(508))
                .device_model_number(ux::u14::new(7156))
                .software_version([
                    ux::u7::new(0x01),
                    ux::u7::new(0x06),
                    ux::u7::new(0x05),
                    ux::u7::new(0x31),
                ])
                .protocol_negotiation_supported(true)
                .profile_configuration_supported(true)
                .property_exchange_supported(true)
                .max_sysex_message_size(ux::u28::new(176315622))
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn sysex8_group() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().group(), ux::u4::new(0x8));
    }

    #[test]
    fn sysex8_source() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().source(), ux::u28::new(196099328));
    }

    #[test]
    fn sysex8_device_manufacturer() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().device_manufacturer(), ux::u21::new(2054957));
    }

    #[test]
    fn sysex8_device_family() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().device_family(), ux::u14::new(508));
    }

    #[test]
    fn sysex8_device_model() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().device_model_number(), ux::u14::new(7156));
    }

    #[test]
    fn sysex8_software_version() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().software_version(), [
                    ux::u7::new(0x01),
                    ux::u7::new(0x06),
                    ux::u7::new(0x05),
                    ux::u7::new(0x31),
                ]);
    }

    #[test]
    fn sysex8_protocol_negotiation_supported() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().protocol_negotiation_supported(), true);
    }

    #[test]
    fn sysex8_property_exchange_supported() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().property_exchange_supported(), true);
    }

    #[test]
    fn sysex8_profile_configuration_supported() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().profile_configuration_supported(), true);
    }

    #[test]
    fn sysex8_max_sysex_size() {
        assert_eq!(DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_317E,
                0x7F0D_7001,
                0x007A_405D,
                0x7F7F_7F7F,
                0x582E_312D,
                0x367D_7C03,
                0x7437_0106,
                0x0531_0E66,
                0x5834_3139,
                0x0954_0000,
                0x0000_0000,
                0x0000_0000,
            ]).unwrap().max_sysex_message_size(), ux::u28::new(176315622));
    }

    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::builder(&mut [0x0; 10])
                .group(ux::u4::new(0x8))
                .source(ux::u28::new(196099328))
                .device_manufacturer(ux::u21::new(2054957))
                .device_family(ux::u14::new(508))
                .device_model_number(ux::u14::new(7156))
                .software_version([
                    ux::u7::new(0x01),
                    ux::u7::new(0x06),
                    ux::u7::new(0x05),
                    ux::u7::new(0x31),
                ])
                .protocol_negotiation_supported(true)
                .profile_configuration_supported(true)
                .property_exchange_supported(true)
                .max_sysex_message_size(ux::u28::new(176315622))
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]),
        );
    }

    #[test]
    fn sysex7_group() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().group(), ux::u4::new(0x8));
    }

    #[test]
    fn sysex7_source() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().source(), ux::u28::new(196099328));
    }

    #[test]
    fn sysex7_device_manufacturer() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().device_manufacturer(), ux::u21::new(2054957));
    }

    #[test]
    fn sysex7_device_family() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().device_family(), ux::u14::new(508));
    }

    #[test]
    fn sysex7_device_model() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().device_model_number(), ux::u14::new(7156));
    }

    #[test]
    fn sysex7_software_version() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().software_version(), [
                    ux::u7::new(0x01),
                    ux::u7::new(0x06),
                    ux::u7::new(0x05),
                    ux::u7::new(0x31),
                ]);
    }

    #[test]
    fn sysex7_protocol_negotiation_supported() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().protocol_negotiation_supported(), true);
    }

    #[test]
    fn sysex7_property_exchange_supported() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().property_exchange_supported(), true);
    }

    #[test]
    fn sysex7_profile_configuration_supported() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().profile_configuration_supported(), true);
    }

    #[test]
    fn sysex7_max_sysex_size() {
        assert_eq!(DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_7E7F,
                0x0D70_0100,
                0x3826_7A40,
                0x5D7F_7F7F,
                0x3826_7F2D,
                0x367D_7C03,
                0x3826_7437,
                0x0106_0531,
                0x3835_0E66,
                0x3909_5400,
            ]).unwrap().max_sysex_message_size(), ux::u28::new(176315622));
    }
}
