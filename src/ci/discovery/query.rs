use super::*;

const STATUS: u7 = u7::new(0x70);

pub struct DiscoveryQueryBorrowed<'a>(DiscoveryBorrowed<'a>);
pub struct DiscoveryQueryBorrowedBuilder<'a>(DiscoveryBorrowedBuilder<'a>);
pub trait DiscoveryQuery: ByteData {
    fn target_muid(&self) -> u28;
    fn device_manufacturer(&self) -> [u7; 3];
    fn device_family(&self) -> u14;
    fn device_family_model_number(&self) -> u14;
    fn software_version(&self) -> [u7; 4];
    fn profile_configuration_supported(&self) -> bool;
    fn property_exchange_supported(&self) -> bool;
    fn process_inquiry_supported(&self) -> bool;
    fn max_sysex_size(&self) -> u28;
    fn initiator_output_path_id(&self) -> u7;
}

impl<'a> ByteData for DiscoveryQueryBorrowed<'a> {
    fn byte_data(&self) -> &[u8] {
        self.0.byte_data()
    }
}

impl<'a> DiscoveryQueryBorrowed<'a> {
    pub fn builder(buffer: &mut [u8]) -> DiscoveryQueryBorrowedBuilder {
        DiscoveryQueryBorrowedBuilder::new(buffer)
    }
}

impl<'a> DiscoveryQuery for DiscoveryQueryBorrowed<'a> {
    fn target_muid(&self) -> u28 {
        self.0.target_muid()
    }
    fn device_manufacturer(&self) -> [u7; 3] {
        self.0.device_manufacturer()
    }
    fn device_family(&self) -> u14 {
        self.0.device_family()
    }
    fn device_family_model_number(&self) -> u14 {
        self.0.device_family_model_number()
    }
    fn software_version(&self) -> [u7; 4] {
        self.0.software_version()
    }
    fn profile_configuration_supported(&self) -> bool {
        self.0.profile_configuration_supported()
    }
    fn property_exchange_supported(&self) -> bool {
        self.0.property_exchange_supported()
    }
    fn process_inquiry_supported(&self) -> bool {
        self.0.process_inquiry_supported()
    }
    fn max_sysex_size(&self) -> u28 {
        self.0.max_sysex_size()
    }
    fn initiator_output_path_id(&self) -> u7 {
        self.0.initiator_output_path_id()
    }
}

impl<'a> Ci for DiscoveryQueryBorrowed<'a> {}

impl<'a> DiscoveryQueryBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self(DiscoveryBorrowedBuilder::new(buffer))
    }

    pub fn source(mut self, v: u28) -> Self {
        self.0 = self.0.source(v);
        self
    }

    pub fn device_manufacturer(mut self, v: [u7; 3]) -> Self {
        self.0 = self.0.device_manufacturer(v);
        self
    }

    pub fn device_family(mut self, v: u14) -> Self {
        self.0 = self.0.device_family(v);
        self
    }

    pub fn device_family_model_number(mut self, v: u14) -> Self {
        self.0 = self.0.device_family_model_number(v);
        self
    }

    pub fn software_version(mut self, v: [u7; 4]) -> Self {
        self.0 = self.0.software_version(v);
        self
    }

    pub fn profile_configuration_supported(mut self, v: bool) -> Self {
        self.0 = self.0.profile_configuration_supported(v);
        self
    }

    pub fn property_exchange_supported(mut self, v: bool) -> Self {
        self.0 = self.0.property_exchange_supported(v);
        self
    }

    pub fn process_inquiry_supported(mut self, v: bool) -> Self {
        self.0 = self.0.process_inquiry_supported(v);
        self
    }

    pub fn max_sysex_size(mut self, v: u28) -> Self {
        self.0 = self.0.max_sysex_size(v);
        self
    }

    pub fn initiator_output_path_id(mut self, v: u7) -> Self {
        self.0 = self.0.initiator_output_path_id(v);
        self
    }

    pub fn build(mut self) -> Result<DiscoveryQueryBorrowed<'a>> {
        self.0 = self.0.destination(u28::max_value());
        self.0.standard_data.sysex_sub_id2 = Some(STATUS);
        match self.0.build() {
            Ok(m) => Ok(DiscoveryQueryBorrowed(m)),
            Err(e) => Err(e),
        }
    }
}

impl<'a> FromByteData<'a> for DiscoveryQueryBorrowed<'a> {
    type Target = Self;
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        DiscoveryBorrowed::validate_byte_data(buffer)?;
        Ok(())
    }
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        Self(DiscoveryBorrowed::from_byte_data_unchecked(buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buffer::Bytes,
        ci::VERSION,
        util::{debug, RandomBuffer},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            debug::ByteData(
                DiscoveryQueryBorrowed::builder(&mut Bytes::random_buffer::<32>())
                    .source(u28::new(0xA87A1F2))
                    .device_manufacturer([u7::new(0x6D), u7::new(0x18), u7::new(0x6)])
                    .device_family(u14::new(0x337C))
                    .device_family_model_number(u14::new(0x1AC5))
                    .software_version([u7::new(0x24), u7::new(0x46), u7::new(0x05), u7::new(0x55)])
                    .process_inquiry_supported(true)
                    .property_exchange_supported(true)
                    .profile_configuration_supported(true)
                    .max_sysex_size(u28::new(0xB187797))
                    .initiator_output_path_id(u7::new(0x1A))
                    .build()
                    .unwrap()
                    .byte_data()
            ),
            debug::ByteData(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ]),
        );
    }

    #[test]
    fn source() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .source(),
            u28::new(0xA87A1F2),
        );
    }

    #[test]
    fn device_manufacturer() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .device_manufacturer(),
            [u7::new(0x6D), u7::new(0x18), u7::new(0x6)],
        );
    }

    #[test]
    fn device_family() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .device_family(),
            u14::new(0x337C),
        );
    }

    #[test]
    fn device_family_model_number() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .device_family_model_number(),
            u14::new(0x1AC5),
        );
    }

    #[test]
    fn software_version() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .software_version(),
            [u7::new(0x24), u7::new(0x46), u7::new(0x05), u7::new(0x55)],
        );
    }

    #[test]
    fn process_inquiry_supported() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .process_inquiry_supported(),
            true,
        );
    }

    #[test]
    fn property_exchange_supported() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .property_exchange_supported(),
            true,
        );
    }

    #[test]
    fn profile_configuration_supported() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .profile_configuration_supported(),
            true,
        );
    }

    #[test]
    fn max_sysex_size() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .max_sysex_size(),
            u28::new(0xB187797),
        );
    }

    #[test]
    fn initiator_output_path_id() {
        assert_eq!(
            DiscoveryQueryBorrowed::from_byte_data(&[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                VERSION.into(),
                0x72,
                0x43,
                0x1E,
                0x54,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x6D,
                0x18,
                0x06,
                0x7C,
                0x66,
                0x45,
                0x35,
                0x24,
                0x46,
                0x05,
                0x55,
                0b0001_1100,
                0x17,
                0x6F,
                0x61,
                0x58,
                0x1A,
                0xF7,
            ])
            .unwrap()
            .initiator_output_path_id(),
            u7::new(0x1A),
        );
    }
}
