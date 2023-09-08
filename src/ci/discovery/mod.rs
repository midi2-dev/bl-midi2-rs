use crate::{
    ci::{helpers as ci_helpers, DeviceId, SYSEX_END},
    error::Error,
    message::system_exclusive_8bit as sysex8,
    result::Result,
    util::{BitOps, Encode7Bit, Truncate},
    *,
};

pub mod query;
pub mod reply;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiscoveryMessage<'a, Repr, const STATUS: u8>(Repr, core::marker::PhantomData<&'a u8>)
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>;

#[repr(usize)]
enum DataOffsets {
    DeviceManufacturer = ci_helpers::STANDARD_DATA_SIZE,
    DeviceFamily = DataOffsets::DeviceManufacturer as usize + 3,
    DeviceFamilyModelNumber = DataOffsets::DeviceFamily as usize + 2,
    SoftwareVersion = DataOffsets::DeviceFamilyModelNumber as usize + 2,
    CiSupportFlags = DataOffsets::SoftwareVersion as usize + 4,
    MaxSysexSize = DataOffsets::CiSupportFlags as usize + 1,
    SysexEnd = DataOffsets::MaxSysexSize as usize + 4,
}

impl<'a, Repr, const STATUS: u8> DiscoveryMessage<'a, Repr, STATUS>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    pub fn source(&self) -> u28 {
        ci_helpers::source_from_payload(self.0.payload())
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
}

impl<'a, Repr, const STATUS: u8> Message<'a> for DiscoveryMessage<'a, Repr, STATUS>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    fn data(&self) -> &'a [u32] {
        self.0.data()
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        DiscoveryMessage(
            <Repr as Message<'a>>::from_data_unchecked(data),
            Default::default(),
        )
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        let messages = ci_helpers::validate_sysex::<'a, Repr>(buffer, STATUS)?;
        let mut payload = messages.payload();
        let Some(SYSEX_END) = payload.nth(DataOffsets::SysexEnd as usize) else {
            return Err(Error::InvalidData);
        };
        let None = payload.next() else {
            return Err(Error::InvalidData);
        };
        Ok(())
    }
}

impl<'a, Repr, const STATUS: u8> Buildable<'a> for DiscoveryMessage<'a, Repr, STATUS>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    type Builder = DiscoveryBuilder<'a, Repr, STATUS>;
}

impl<'a, Repr, const STATUS: u8> GroupedMessage<'a> for DiscoveryMessage<'a, Repr, STATUS>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    fn group(&self) -> u4 {
        self.0.group()
    }
}

impl<'a, const STATUS: u8> StreamedMessage<'a>
    for DiscoveryMessage<'a, sysex8::Sysex8MessageGroup<'a>, STATUS>
{
    fn stream_id(&self) -> u8 {
        self.0.stream_id()
    }
}

pub struct DiscoveryBuilder<'a, Repr, const STATUS: u8>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
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

impl<'a, Repr, const STATUS: u8> DiscoveryBuilder<'a, Repr, STATUS>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    pub fn source(mut self, source: u28) -> Self {
        self.source = source;
        self
    }
    pub fn destination(mut self, dest: u28) -> Self {
        self.destination = dest;
        self
    }
    pub fn device_manufacturer(mut self, device_manufacturer: u21) -> Self {
        self.device_manufacturer = device_manufacturer;
        self
    }
    pub fn device_family(mut self, device_family: u14) -> Self {
        self.device_family = device_family;
        self
    }
    pub fn device_model_number(mut self, device_model_number: u14) -> Self {
        self.device_model_number = device_model_number;
        self
    }
    pub fn software_version(mut self, software_version: [u7; 4]) -> Self {
        self.software_version = software_version;
        self
    }
    pub fn protocol_negotiation_supported(mut self, protocol_negotiation_supported: bool) -> Self {
        self.protocol_negotiation_supported = protocol_negotiation_supported;
        self
    }
    pub fn profile_configuration_supported(
        mut self,
        profile_configuration_supported: bool,
    ) -> Self {
        self.profile_configuration_supported = profile_configuration_supported;
        self
    }
    pub fn property_exchange_supported(mut self, property_exchange_supported: bool) -> Self {
        self.property_exchange_supported = property_exchange_supported;
        self
    }
    pub fn max_sysex_message_size(mut self, max_sysex_message_size: u28) -> Self {
        self.max_sysex_message_size = max_sysex_message_size;
        self
    }
}

impl<'a, Repr, const STATUS: u8> Builder<'a> for DiscoveryBuilder<'a, Repr, STATUS>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    type Message = DiscoveryMessage<'a, Repr, STATUS>;
    fn build(self) -> Result<DiscoveryMessage<'a, Repr, STATUS>> {
        let payload = ci_helpers::StandardDataIterator::<'a, Repr>::new(
            DeviceId::MidiPort,
            STATUS,
            self.source,
            self.destination,
        );

        let u8_to_byte = <<Repr as Buildable<'a>>::Builder as SysexBuilder<'a>>::Byte::from_u8;
        let u7_to_byte = |v: u7| u8_to_byte(u8::from(v));

        let device_manufacturer_array = self.device_manufacturer.to_u7s();
        let payload = payload.chain(device_manufacturer_array.iter().cloned().map(u7_to_byte));

        let device_family_array = self.device_family.to_u7s();
        let payload = payload.chain(device_family_array.iter().cloned().map(u7_to_byte));

        let device_model_number_array = self.device_model_number.to_u7s();
        let payload = payload.chain(device_model_number_array.iter().cloned().map(u7_to_byte));

        let payload = payload.chain(self.software_version.iter().cloned().map(u7_to_byte));

        let support_flags = {
            let mut bits: u8 = 0x0;
            bits.set_bit(6, self.protocol_negotiation_supported);
            bits.set_bit(5, self.profile_configuration_supported);
            bits.set_bit(4, self.property_exchange_supported);
            bits
        };
        let payload = payload.chain(core::iter::once(u8_to_byte(support_flags)));

        let max_sysex_size_array = self.max_sysex_message_size.to_u7s();
        let payload = payload.chain(max_sysex_size_array.iter().cloned().map(u7_to_byte));

        match self.builder.payload(payload).build() {
            Ok(messages) => Ok(DiscoveryMessage(messages, Default::default())),
            Err(e) => Err(e),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        Self {
            builder: <Repr as Buildable<'a>>::Builder::new(buffer),
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
}

impl<'a, Repr, const STATUS: u8> GroupedBuilder<'a> for DiscoveryBuilder<'a, Repr, STATUS>
where
    Repr: 'a + SysexMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexBuilder<'a>,
{
    fn group(mut self, group: u4) -> Self {
        self.builder = self.builder.group(group);
        self
    }
}

impl<'a, const STATUS: u8> StreamedBuilder<'a>
    for DiscoveryBuilder<'a, sysex8::Sysex8MessageGroup<'a>, STATUS>
{
    fn stream_id(mut self, id: u8) -> Self {
        self.builder = self.builder.stream_id(id);
        self
    }
}
