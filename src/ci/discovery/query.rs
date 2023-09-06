use super::*;

const STATUS: u8 = 0x70;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiscoveryQueryMessage<'a, Repr>(DiscoveryMessage<'a, Repr, STATUS>)
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>;

impl<'a, Repr> DiscoveryQueryMessage<'a, Repr>
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    pub fn source(&self) -> u28 {
        self.0.source()
    }
    // todo: array of 3 u7s
    pub fn device_manufacturer(&self) -> u21 {
        self.0.device_manufacturer()
    }
    pub fn device_family(&self) -> u14 {
        self.0.device_family()
    }
    pub fn device_model_number(&self) -> u14 {
        self.0.device_model_number()
    }
    pub fn software_version(&self) -> [u7; 4] {
        self.0.software_version()
    }
    pub fn protocol_negotiation_supported(&self) -> bool {
        self.0.protocol_negotiation_supported()
    }
    pub fn profile_configuration_supported(&self) -> bool {
        self.0.profile_configuration_supported()
    }
    pub fn property_exchange_supported(&self) -> bool {
        self.0.property_exchange_supported()
    }
    pub fn max_sysex_message_size(&self) -> u28 {
        self.0.max_sysex_message_size()
    }
}

impl<'a, Repr> Message<'a> for DiscoveryQueryMessage<'a, Repr>
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    fn data(&self) -> &'a [u32] {
        self.0.data()
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        Self(DiscoveryMessage::<'a, Repr, STATUS>::from_data_unchecked(
            data,
        ))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        DiscoveryMessage::<'a, Repr, STATUS>::validate_data(buffer)
    }
}

impl<'a, Repr> Buildable<'a> for DiscoveryQueryMessage<'a, Repr>
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    type Builder = DiscoveryQueryBuilder<'a, Repr>;
}

impl<'a, Repr> GroupedMessage<'a> for DiscoveryQueryMessage<'a, Repr>
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    fn group(&self) -> u4 {
        self.0.group()
    }
}

impl<'a> StreamedMessage<'a> for DiscoveryQueryMessage<'a, sysex8::Sysex8MessageGroup<'a>> {
    fn stream_id(&self) -> u8 {
        self.0.stream_id()
    }
}

pub struct DiscoveryQueryBuilder<'a, Repr>(DiscoveryBuilder<'a, Repr, STATUS>)
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>;

impl<'a, Repr> DiscoveryQueryBuilder<'a, Repr>
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    pub fn source(mut self, source: u28) -> Self {
        self.0 = self.0.source(source);
        self
    }
    pub fn device_manufacturer(mut self, device_manufacturer: u21) -> Self {
        self.0 = self.0.device_manufacturer(device_manufacturer);
        self
    }
    pub fn device_family(mut self, device_family: u14) -> Self {
        self.0 = self.0.device_family(device_family);
        self
    }
    pub fn device_model_number(mut self, device_model_number: u14) -> Self {
        self.0 = self.0.device_model_number(device_model_number);
        self
    }
    pub fn software_version(mut self, software_version: [u7; 4]) -> Self {
        self.0 = self.0.software_version(software_version);
        self
    }
    pub fn protocol_negotiation_supported(mut self, protocol_negotiation_supported: bool) -> Self {
        self.0 = self
            .0
            .protocol_negotiation_supported(protocol_negotiation_supported);
        self
    }
    pub fn profile_configuration_supported(
        mut self,
        profile_configuration_supported: bool,
    ) -> Self {
        self.0 = self
            .0
            .profile_configuration_supported(profile_configuration_supported);
        self
    }
    pub fn property_exchange_supported(mut self, property_exchange_supported: bool) -> Self {
        self.0 = self
            .0
            .property_exchange_supported(property_exchange_supported);
        self
    }
    pub fn max_sysex_message_size(mut self, max_sysex_message_size: u28) -> Self {
        self.0 = self.0.max_sysex_message_size(max_sysex_message_size);
        self
    }
}

impl<'a, Repr> Builder<'a> for DiscoveryQueryBuilder<'a, Repr>
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    type Message = DiscoveryQueryMessage<'a, Repr>;
    fn build(self) -> Result<DiscoveryQueryMessage<'a, Repr>> {
        match self.0.build() {
            Ok(message) => Ok(DiscoveryQueryMessage(message)),
            Err(e) => Err(e),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        let mut builder = DiscoveryBuilder::<'a, Repr, STATUS>::new(buffer);
        builder = builder.destination(u28::max_value());
        Self(builder)
    }
}

impl<'a, Repr> GroupedBuilder<'a> for DiscoveryQueryBuilder<'a, Repr>
where
    Repr: 'a + SysexGroupMessage<'a> + GroupedMessage<'a> + Buildable<'a>,
    <Repr as Buildable<'a>>::Builder: GroupedBuilder<'a> + SysexGroupBuilder<'a>,
{
    fn group(mut self, group: u4) -> Self {
        self.0 = self.0.group(group);
        self
    }
}

impl<'a> StreamedBuilder<'a> for DiscoveryQueryBuilder<'a, sysex8::Sysex8MessageGroup<'a>> {
    fn stream_id(mut self, id: u8) -> Self {
        self.0 = self.0.stream_id(id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        message::{system_exclusive_7bit as sysex7, system_exclusive_8bit as sysex8},
        util::{debug, random_buffer},
    };

    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(
                DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::builder(&mut random_buffer::<
                    12,
                >(
                ))
                .group(u4::new(0x8))
                .stream_id(0x31)
                .source(u28::new(196099328))
                .device_manufacturer(u21::new(2054957))
                .device_family(u14::new(508))
                .device_model_number(u14::new(7156))
                .software_version([u7::new(0x01), u7::new(0x06), u7::new(0x05), u7::new(0x31),])
                .protocol_negotiation_supported(true)
                .profile_configuration_supported(true)
                .property_exchange_supported(true)
                .max_sysex_message_size(u28::new(176315622))
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn sysex8_group() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .group(),
            u4::new(0x8)
        );
    }

    #[test]
    fn sysex8_source() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .source(),
            u28::new(196099328)
        );
    }

    #[test]
    fn sysex8_device_manufacturer() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .device_manufacturer(),
            u21::new(2054957)
        );
    }

    #[test]
    fn sysex8_device_family() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .device_family(),
            u14::new(508)
        );
    }

    #[test]
    fn sysex8_device_model() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .device_model_number(),
            u14::new(7156)
        );
    }

    #[test]
    fn sysex8_software_version() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .software_version(),
            [u7::new(0x01), u7::new(0x06), u7::new(0x05), u7::new(0x31),]
        );
    }

    #[test]
    fn sysex8_protocol_negotiation_supported() {
        assert!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .protocol_negotiation_supported()
        );
    }

    #[test]
    fn sysex8_property_exchange_supported() {
        assert!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .property_exchange_supported()
        );
    }

    #[test]
    fn sysex8_profile_configuration_supported() {
        assert!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .profile_configuration_supported()
        );
    }

    #[test]
    fn sysex8_max_sysex_size() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
                0x581E_31F0,
                0x7E7F_0D70,
                0x0100_7A40,
                0x5D7F_7F7F,
                0x582E_317F,
                0x2D36_7D7C,
                0x0374_3701,
                0x0605_310E,
                0x5836_3166,
                0x3909_54F7,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .max_sysex_message_size(),
            u28::new(176315622)
        );
    }

    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(
                DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::builder(&mut random_buffer::<
                    12,
                >(
                ))
                .group(u4::new(0x8))
                .source(u28::new(196099328))
                .device_manufacturer(u21::new(2054957))
                .device_family(u14::new(508))
                .device_model_number(u14::new(7156))
                .software_version([u7::new(0x01), u7::new(0x06), u7::new(0x05), u7::new(0x31),])
                .protocol_negotiation_supported(true)
                .profile_configuration_supported(true)
                .property_exchange_supported(true)
                .max_sysex_message_size(u28::new(176315622))
                .build()
                .unwrap()
                .data(),
            ),
            debug::Data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3826_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn sysex7_group() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .group(),
            u4::new(0x8)
        );
    }

    #[test]
    fn sysex7_source() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .source(),
            u28::new(196099328)
        );
    }

    #[test]
    fn sysex7_device_manufacturer() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .device_manufacturer(),
            u21::new(2054957)
        );
    }

    #[test]
    fn sysex7_device_family() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .device_family(),
            u14::new(508)
        );
    }

    #[test]
    fn sysex7_device_model() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .device_model_number(),
            u14::new(7156)
        );
    }

    #[test]
    fn sysex7_software_version() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .software_version(),
            [u7::new(0x01), u7::new(0x06), u7::new(0x05), u7::new(0x31),]
        );
    }

    #[test]
    fn sysex7_protocol_negotiation_supported() {
        assert!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .protocol_negotiation_supported()
        );
    }

    #[test]
    fn sysex7_property_exchange_supported() {
        assert!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .property_exchange_supported()
        );
    }

    #[test]
    fn sysex7_profile_configuration_supported() {
        assert!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .profile_configuration_supported()
        );
    }

    #[test]
    fn sysex7_max_sysex_size() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
                0x3816_F07E,
                0x7F0D_7001,
                0x3826_007A,
                0x405D_7F7F,
                0x3826_7F7F,
                0x2D36_7D7C,
                0x3826_0374,
                0x3701_0605,
                0x3836_310E,
                0x6639_0954,
                0x3831_F700,
                0x0000_0000,
            ])
            .unwrap()
            .max_sysex_message_size(),
            u28::new(176315622)
        );
    }
}
