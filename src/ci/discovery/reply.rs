use super::*;

const STATUS: u7 = u7::new(0x71);

pub struct DiscoveryReplyBorrowed<'a>(DiscoveryBorrowed<'a>);
pub struct DiscoveryReplyBorrowedBuilder<'a>(DiscoveryBorrowedBuilder<'a>);
pub trait DiscoveryReply: ByteData {
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

impl<'a> ByteData for DiscoveryReplyBorrowed<'a> {
    fn byte_data(&self) -> &[u8] {
        self.0.byte_data()
    }
}

impl<'a> DiscoveryReplyBorrowed<'a> {
    pub fn builder(buffer: &mut [u8]) -> DiscoveryReplyBorrowedBuilder {
        DiscoveryReplyBorrowedBuilder::new(buffer)
    }
}

impl<'a> DiscoveryReply for DiscoveryReplyBorrowed<'a> {
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

impl<'a> Ci for DiscoveryReplyBorrowed<'a> {}

impl<'a> DiscoveryReplyBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self(DiscoveryBorrowedBuilder::new(buffer))
    }

    pub fn source(mut self, v: u28) -> Self {
        self.0 = self.0.source(v);
        self
    }

    pub fn destination(mut self, v: u28) -> Self {
        self.0 = self.0.destination(v);
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

    pub fn build(mut self) -> Result<DiscoveryReplyBorrowed<'a>> {
        self.0 = self.0.destination(u28::max_value());
        self.0.standard_data.sysex_sub_id2 = Some(STATUS);
        match self.0.build() {
            Ok(m) => Ok(DiscoveryReplyBorrowed(m)),
            Err(e) => Err(e),
        }
    }
}

impl<'a> FromByteData<'a> for DiscoveryReplyBorrowed<'a> {
    type Target = Self;
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        DiscoveryBorrowed::validate_byte_data(buffer)?;
        Ok(())
    }
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        Self(DiscoveryBorrowed::from_byte_data_unchecked(buffer))
    }
}
