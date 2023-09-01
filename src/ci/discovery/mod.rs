use crate::{
    ci::{helpers as ci_helpers, DeviceId},
    error::Error,
    message::{sysex, system_exclusive_7bit as sysex7, system_exclusive_8bit as sysex8},
    result::Result,
    util::{BitOps, Encode7Bit, Truncate},
    *,
};

pub mod query;
pub mod reply;

#[derive(Clone, PartialEq, Eq, Debug)]
struct DiscoveryMessage<Repr: sysex::SysexMessages, const STATUS: u8>(Repr);

#[repr(usize)]
enum DataOffsets {
    DeviceManufacturer = ci_helpers::STANDARD_DATA_SIZE,
    DeviceFamily = DataOffsets::DeviceManufacturer as usize + 3,
    DeviceFamilyModelNumber = DataOffsets::DeviceFamily as usize + 2,
    SoftwareVersion = DataOffsets::DeviceFamilyModelNumber as usize + 2,
    CiSupportFlags = DataOffsets::SoftwareVersion as usize + 4,
    MaxSysexSize = DataOffsets::CiSupportFlags as usize + 1,
}

impl<'a, const STATUS: u8> DiscoveryMessage<sysex8::Sysex8MessageGroup<'a>, STATUS> {
    pub fn builder(
        buffer: &'a mut [u32],
    ) -> DiscoveryBuilder<sysex8::Sysex8MessageGroup<'a>, STATUS> {
        DiscoveryBuilder::<sysex8::Sysex8MessageGroup<'a>, STATUS>::new(buffer)
    }
    pub fn group(&self) -> u4 {
        self.0.group()
    }
    pub fn source(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        u28::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn destination(&self) -> u28 {
        ci_helpers::destination_from_payload(self.0.payload())
    }
    pub fn device_manufacturer(&self) -> u21 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceManufacturer as usize - 1);
        u21::from_u7s(&[
            payload.next().unwrap(),
            payload.next().unwrap(),
            payload.next().unwrap(),
        ])
    }
    pub fn device_family(&self) -> u14 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceFamily as usize - 1);
        u14::from_u7s(&[payload.next().unwrap(), payload.next().unwrap()])
    }
    pub fn device_model_number(&self) -> u14 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceFamilyModelNumber as usize - 1);
        u14::from_u7s(&[payload.next().unwrap(), payload.next().unwrap()])
    }
    pub fn software_version(&self) -> [u7; 4] {
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
        payload
            .nth(DataOffsets::CiSupportFlags as usize)
            .unwrap()
            .bit(6)
    }
    pub fn profile_configuration_supported(&self) -> bool {
        let mut payload = self.0.payload();
        payload
            .nth(DataOffsets::CiSupportFlags as usize)
            .unwrap()
            .bit(5)
    }
    pub fn property_exchange_supported(&self) -> bool {
        let mut payload = self.0.payload();
        payload
            .nth(DataOffsets::CiSupportFlags as usize)
            .unwrap()
            .bit(4)
    }
    pub fn max_sysex_message_size(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::MaxSysexSize as usize - 1);
        u28::from_u7s(&[
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
        Ok(DiscoveryMessage(messages))
    }
}

impl<'a, const STATUS: u8> DiscoveryMessage<sysex7::Sysex7MessageGroup<'a>, STATUS> {
    pub fn builder(
        buffer: &'a mut [u32],
    ) -> DiscoveryBuilder<sysex7::Sysex7MessageGroup<'a>, STATUS> {
        DiscoveryBuilder::<sysex7::Sysex7MessageGroup<'a>, STATUS>::new(buffer)
    }
    pub fn group(&self) -> u4 {
        self.0.group()
    }
    pub fn source(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(4);
        u28::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn destination(&self) -> u28 {
        ci_helpers::destination_from_payload(self.0.payload().map(u8::from))
    }
    pub fn device_manufacturer(&self) -> u21 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceManufacturer as usize - 1);
        u21::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn device_family(&self) -> u14 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceFamily as usize - 1);
        u14::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn device_model_number(&self) -> u14 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::DeviceFamilyModelNumber as usize - 1);
        u14::from_u7s(&[
            payload.next().unwrap().into(),
            payload.next().unwrap().into(),
        ])
    }
    pub fn software_version(&self) -> [u7; 4] {
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
        payload
            .nth(DataOffsets::CiSupportFlags as usize)
            .unwrap()
            .into()
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
    pub fn max_sysex_message_size(&self) -> u28 {
        let mut payload = self.0.payload();
        payload.nth(DataOffsets::MaxSysexSize as usize - 1);
        u28::from_u7s(&[
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
        Ok(DiscoveryMessage(messages))
    }
}

struct DiscoveryBuilder<Repr: sysex::SysexMessages, const STATUS: u8> {
    source: u28,
    destination: u28,
    device_manufacturer: u21,
    device_family: u14,
    device_model_number: u14,
    software_version: [u7; 4],
    protocol_negotiation_supported: bool,
    profile_configuration_supported: bool,
    property_exchange_supported: bool,
    max_sysex_message_size: u28,
    builder: Repr::Builder,
}

impl<'a, const STATUS: u8> DiscoveryBuilder<sysex8::Sysex8MessageGroup<'a>, STATUS> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Self {
            builder: sysex8::Sysex8MessageGroupBuilder::new(buffer),
            source: Default::default(),
            destination: Default::default(),
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
    pub fn group(&mut self, group: u4) -> &mut Self {
        self.builder.group(group);
        self
    }
    pub fn source(&mut self, source: u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn destination(&mut self, dest: u28) -> &mut Self {
        self.destination = dest;
        self
    }
    pub fn device_manufacturer(&mut self, device_manufacturer: u21) -> &mut Self {
        self.device_manufacturer = device_manufacturer;
        self
    }
    pub fn device_family(&mut self, device_family: u14) -> &mut Self {
        self.device_family = device_family;
        self
    }
    pub fn device_model_number(&mut self, device_model_number: u14) -> &mut Self {
        self.device_model_number = device_model_number;
        self
    }
    pub fn software_version(&mut self, software_version: [u7; 4]) -> &mut Self {
        self.software_version = software_version;
        self
    }
    pub fn protocol_negotiation_supported(
        &mut self,
        protocol_negotiation_supported: bool,
    ) -> &mut Self {
        self.protocol_negotiation_supported = protocol_negotiation_supported;
        self
    }
    pub fn profile_configuration_supported(
        &mut self,
        profile_configuration_supported: bool,
    ) -> &mut Self {
        self.profile_configuration_supported = profile_configuration_supported;
        self
    }
    pub fn property_exchange_supported(&mut self, property_exchange_supported: bool) -> &mut Self {
        self.property_exchange_supported = property_exchange_supported;
        self
    }
    pub fn max_sysex_message_size(&mut self, max_sysex_message_size: u28) -> &mut Self {
        self.max_sysex_message_size = max_sysex_message_size;
        self
    }
    pub fn build(&'a mut self) -> Result<DiscoveryMessage<sysex8::Sysex8MessageGroup<'a>, STATUS>> {
        let payload = ci_helpers::StandardDataIterator::new(
            DeviceId::MidiPort,
            STATUS,
            self.source,
            self.destination,
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
            Ok(messages) => Ok(DiscoveryMessage(messages)),
            Err(e) => Err(e),
        }
    }
}

impl<'a, const STATUS: u8> DiscoveryBuilder<sysex7::Sysex7MessageGroup<'a>, STATUS> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Self {
            builder: sysex7::Sysex7MessageGroupBuilder::new(buffer),
            source: Default::default(),
            destination: Default::default(),
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
    pub fn group(&mut self, group: u4) -> &mut Self {
        self.builder.group(group);
        self
    }
    pub fn source(&mut self, source: u28) -> &mut Self {
        self.source = source;
        self
    }
    pub fn destination(&mut self, dest: u28) -> &mut Self {
        self.destination = dest;
        self
    }
    pub fn device_manufacturer(&mut self, device_manufacturer: u21) -> &mut Self {
        self.device_manufacturer = device_manufacturer;
        self
    }
    pub fn device_family(&mut self, device_family: u14) -> &mut Self {
        self.device_family = device_family;
        self
    }
    pub fn device_model_number(&mut self, device_model_number: u14) -> &mut Self {
        self.device_model_number = device_model_number;
        self
    }
    pub fn software_version(&mut self, software_version: [u7; 4]) -> &mut Self {
        self.software_version = software_version;
        self
    }
    pub fn protocol_negotiation_supported(
        &mut self,
        protocol_negotiation_supported: bool,
    ) -> &mut Self {
        self.protocol_negotiation_supported = protocol_negotiation_supported;
        self
    }
    pub fn profile_configuration_supported(
        &mut self,
        profile_configuration_supported: bool,
    ) -> &mut Self {
        self.profile_configuration_supported = profile_configuration_supported;
        self
    }
    pub fn property_exchange_supported(&mut self, property_exchange_supported: bool) -> &mut Self {
        self.property_exchange_supported = property_exchange_supported;
        self
    }
    pub fn max_sysex_message_size(&mut self, max_sysex_message_size: u28) -> &mut Self {
        self.max_sysex_message_size = max_sysex_message_size;
        self
    }
    pub fn build(&'a mut self) -> Result<DiscoveryMessage<sysex7::Sysex7MessageGroup<'a>, STATUS>> {
        let payload = ci_helpers::StandardDataIterator::new(
            DeviceId::MidiPort,
            STATUS,
            self.source,
            self.destination,
        )
        .map(|v| v.truncate());

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
            Ok(messages) => Ok(DiscoveryMessage(messages)),
            Err(e) => Err(e),
        }
    }
}
