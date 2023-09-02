use super::*;

const STATUS: u8 = 0x70;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiscoveryQueryMessage<Repr: sysex::SysexMessages>(DiscoveryMessage<Repr, STATUS>);

impl<'a> DiscoveryQueryMessage<sysex8::Sysex8MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> DiscoveryQueryBuilder<sysex8::Sysex8MessageGroup<'a>> {
        DiscoveryQueryBuilder::<sysex8::Sysex8MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> u4 {
        self.0.group()
    }
    pub fn source(&self) -> u28 {
        self.0.source()
    }
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
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        // todo assert destination is defaulted
        match DiscoveryMessage::<sysex8::Sysex8MessageGroup<'a>, STATUS>::from_data(data) {
            Ok(message) => Ok(Self(message)),
            Err(e) => Err(e),
        }
    }
}

impl<'a> DiscoveryQueryMessage<sysex7::Sysex7MessageGroup<'a>> {
    pub fn builder(buffer: &'a mut [u32]) -> DiscoveryQueryBuilder<sysex7::Sysex7MessageGroup<'a>> {
        DiscoveryQueryBuilder::<sysex7::Sysex7MessageGroup<'a>>::new(buffer)
    }
    pub fn group(&self) -> u4 {
        self.0.group()
    }
    pub fn source(&self) -> u28 {
        self.0.source()
    }
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
    pub fn data(&self) -> &[u32] {
        self.0.data()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        // todo assert destination is defaulted
        match DiscoveryMessage::<sysex7::Sysex7MessageGroup<'a>, STATUS>::from_data(data) {
            Ok(message) => Ok(Self(message)),
            Err(e) => Err(e),
        }
    }
}

pub struct DiscoveryQueryBuilder<Repr: sysex::SysexMessages>(DiscoveryBuilder<Repr, STATUS>);

impl<'a> DiscoveryQueryBuilder<sysex8::Sysex8MessageGroup<'a>> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        let mut builder = DiscoveryBuilder::<sysex8::Sysex8MessageGroup<'a>, STATUS>::new(buffer);
        builder.destination(u28::max_value());
        Self(builder)
    }
    pub fn stream_id(&mut self, id: u8) -> &mut Self {
        self.0.stream_id(id);
        self
    }
    pub fn group(&mut self, group: u4) -> &mut Self {
        self.0.group(group);
        self
    }
    pub fn source(&mut self, source: u28) -> &mut Self {
        self.0.source(source);
        self
    }
    pub fn device_manufacturer(&mut self, device_manufacturer: u21) -> &mut Self {
        self.0.device_manufacturer(device_manufacturer);
        self
    }
    pub fn device_family(&mut self, device_family: u14) -> &mut Self {
        self.0.device_family(device_family);
        self
    }
    pub fn device_model_number(&mut self, device_model_number: u14) -> &mut Self {
        self.0.device_model_number(device_model_number);
        self
    }
    pub fn software_version(&mut self, software_version: [u7; 4]) -> &mut Self {
        self.0.software_version(software_version);
        self
    }
    pub fn protocol_negotiation_supported(
        &mut self,
        protocol_negotiation_supported: bool,
    ) -> &mut Self {
        self.0
            .protocol_negotiation_supported(protocol_negotiation_supported);
        self
    }
    pub fn profile_configuration_supported(
        &mut self,
        profile_configuration_supported: bool,
    ) -> &mut Self {
        self.0
            .profile_configuration_supported(profile_configuration_supported);
        self
    }
    pub fn property_exchange_supported(&mut self, property_exchange_supported: bool) -> &mut Self {
        self.0
            .property_exchange_supported(property_exchange_supported);
        self
    }
    pub fn max_sysex_message_size(&mut self, max_sysex_message_size: u28) -> &mut Self {
        self.0.max_sysex_message_size(max_sysex_message_size);
        self
    }
    pub fn build(&'a mut self) -> Result<DiscoveryQueryMessage<sysex8::Sysex8MessageGroup<'a>>> {
        match self.0.build() {
            Ok(message) => Ok(DiscoveryQueryMessage(message)),
            Err(e) => Err(e),
        }
    }
}

impl<'a> DiscoveryQueryBuilder<sysex7::Sysex7MessageGroup<'a>> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        let mut builder = DiscoveryBuilder::<sysex7::Sysex7MessageGroup<'a>, STATUS>::new(buffer);
        builder.destination(u28::max_value());
        Self(builder)
    }
    pub fn group(&mut self, group: u4) -> &mut Self {
        self.0.group(group);
        self
    }
    pub fn source(&mut self, source: u28) -> &mut Self {
        self.0.source(source);
        self
    }
    pub fn device_manufacturer(&mut self, device_manufacturer: u21) -> &mut Self {
        self.0.device_manufacturer(device_manufacturer);
        self
    }
    pub fn device_family(&mut self, device_family: u14) -> &mut Self {
        self.0.device_family(device_family);
        self
    }
    pub fn device_model_number(&mut self, device_model_number: u14) -> &mut Self {
        self.0.device_model_number(device_model_number);
        self
    }
    pub fn software_version(&mut self, software_version: [u7; 4]) -> &mut Self {
        self.0.software_version(software_version);
        self
    }
    pub fn protocol_negotiation_supported(
        &mut self,
        protocol_negotiation_supported: bool,
    ) -> &mut Self {
        self.0
            .protocol_negotiation_supported(protocol_negotiation_supported);
        self
    }
    pub fn profile_configuration_supported(
        &mut self,
        profile_configuration_supported: bool,
    ) -> &mut Self {
        self.0
            .profile_configuration_supported(profile_configuration_supported);
        self
    }
    pub fn property_exchange_supported(&mut self, property_exchange_supported: bool) -> &mut Self {
        self.0
            .property_exchange_supported(property_exchange_supported);
        self
    }
    pub fn max_sysex_message_size(&mut self, max_sysex_message_size: u28) -> &mut Self {
        self.0.max_sysex_message_size(max_sysex_message_size);
        self
    }
    pub fn build(&'a mut self) -> Result<DiscoveryQueryMessage<sysex7::Sysex7MessageGroup<'a>>> {
        match self.0.build() {
            Ok(message) => Ok(DiscoveryQueryMessage(message)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{debug, random_buffer};

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
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .software_version(),
            [u7::new(0x01), u7::new(0x06), u7::new(0x05), u7::new(0x31),]
        );
    }

    #[test]
    fn sysex8_protocol_negotiation_supported() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .protocol_negotiation_supported(),
            true
        );
    }

    #[test]
    fn sysex8_property_exchange_supported() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .property_exchange_supported(),
            true
        );
    }

    #[test]
    fn sysex8_profile_configuration_supported() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .profile_configuration_supported(),
            true
        );
    }

    #[test]
    fn sysex8_max_sysex_size() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex8::Sysex8MessageGroup>::from_data(&[
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
                    10,
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
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .software_version(),
            [u7::new(0x01), u7::new(0x06), u7::new(0x05), u7::new(0x31),]
        );
    }

    #[test]
    fn sysex7_protocol_negotiation_supported() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .protocol_negotiation_supported(),
            true
        );
    }

    #[test]
    fn sysex7_property_exchange_supported() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .property_exchange_supported(),
            true
        );
    }

    #[test]
    fn sysex7_profile_configuration_supported() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .profile_configuration_supported(),
            true
        );
    }

    #[test]
    fn sysex7_max_sysex_size() {
        assert_eq!(
            DiscoveryQueryMessage::<sysex7::Sysex7MessageGroup>::from_data(&[
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
            ])
            .unwrap()
            .max_sysex_message_size(),
            u28::new(176315622)
        );
    }
}
