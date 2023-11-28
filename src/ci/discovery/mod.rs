use crate::{
    ci::{helpers, Ci, CiStandardData, DeviceId},
    message::sysex_bytes::{Sysex7BytesBorrowed, Sysex7BytesBorrowedBuilder},
    util::{BitOps, Encode7Bit, Truncate},
    *,
};

pub mod query;
pub mod reply;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveryBorrowed<'a>(Sysex7BytesBorrowed<'a>);

pub struct DiscoveryBorrowedBuilder<'a> {
    sysex_builder: Sysex7BytesBorrowedBuilder<'a>,
    standard_data: CiStandardData,
    discovery_data: [u7; 17],
}

pub trait Discovery: ByteData {
    fn target_muid(&self) -> u28 {
        u28::from_u7s(&self.byte_data()[14..18])
    }
    fn device_manufacturer(&self) -> [u7; 3] {
        let mut ret = [u7::default(); 3];
        let data = self.byte_data();
        for (i, d) in data[14..17].iter().enumerate() {
            ret[i] = d.truncate();
        }
        ret
    }
    fn device_family(&self) -> u14 {
        let data = self.byte_data();
        u14::from_u7s(&data[17..19])
    }
    fn device_family_model_number(&self) -> u14 {
        let data = self.byte_data();
        u14::from_u7s(&data[19..21])
    }
    fn software_version(&self) -> [u7; 4] {
        let mut ret = [u7::default(); 4];
        let data = self.byte_data();
        for (i, d) in data[21..25].iter().enumerate() {
            ret[i] = d.truncate();
        }
        ret
    }
    fn profile_configuration_supported(&self) -> bool {
        let data = self.byte_data();
        data[25].bit(5)
    }
    fn property_exchange_supported(&self) -> bool {
        let data = self.byte_data();
        data[25].bit(4)
    }
    fn process_inquiry_supported(&self) -> bool {
        let data = self.byte_data();
        data[25].bit(3)
    }
    fn max_sysex_size(&self) -> u28 {
        let data = self.byte_data();
        u28::from_u7s(&data[26..30])
    }
    fn initiator_output_path_id(&self) -> u7 {
        let data = self.byte_data();
        data[30].truncate()
    }
}

impl<'a> ByteData for DiscoveryBorrowed<'a> {
    fn byte_data(&self) -> &[u8] {
        self.0.byte_data()
    }
}

impl<'a> Discovery for DiscoveryBorrowed<'a> {}

impl<'a> Ci for DiscoveryBorrowed<'a> {}

impl<'a> FromByteData<'a> for DiscoveryBorrowed<'a> {
    type Target = Self;
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        Sysex7BytesBorrowed::validate_byte_data(buffer)?;
        helpers::validate_ci_standard_bytes(buffer)?;
        if buffer.len() < 32 {
            return Err(Error::InvalidData);
        }
        Ok(())
    }
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        Self(Sysex7BytesBorrowed::from_byte_data_unchecked(buffer))
    }
}

impl<'a> DiscoveryBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            sysex_builder: Sysex7BytesBorrowedBuilder::new(buffer),
            standard_data: CiStandardData {
                device_id: DeviceId::FunctionBlock,
                ..Default::default()
            },
            discovery_data: Default::default(),
        }
    }

    pub fn source(mut self, v: u28) -> Self {
        self.standard_data.source = Some(v);
        self
    }

    pub fn destination(mut self, v: u28) -> Self {
        self.standard_data.destination = Some(v);
        self
    }

    pub fn device_manufacturer(mut self, v: [u7; 3]) -> Self {
        self.discovery_data[0..3].copy_from_slice(&v[..]);
        self
    }

    pub fn device_family(mut self, v: u14) -> Self {
        v.to_u7s(&mut self.discovery_data[3..5]);
        self
    }

    pub fn device_family_model_number(mut self, v: u14) -> Self {
        v.to_u7s(&mut self.discovery_data[5..7]);
        self
    }

    pub fn software_version(mut self, v: [u7; 4]) -> Self {
        self.discovery_data[7..11].copy_from_slice(&v);
        self
    }

    pub fn profile_configuration_supported(mut self, v: bool) -> Self {
        self.discovery_data[11].set_bit(4, v);
        self
    }

    pub fn property_exchange_supported(mut self, v: bool) -> Self {
        self.discovery_data[11].set_bit(3, v);
        self
    }

    pub fn process_inquiry_supported(mut self, v: bool) -> Self {
        self.discovery_data[11].set_bit(2, v);
        self
    }

    pub fn max_sysex_size(mut self, v: u28) -> Self {
        v.to_u7s(&mut self.discovery_data[12..16]);
        self
    }

    pub fn initiator_output_path_id(mut self, v: u7) -> Self {
        self.discovery_data[16] = v;
        self
    }

    pub fn build(mut self) -> Result<DiscoveryBorrowed<'a>> {
        self.sysex_builder = self.sysex_builder.payload(self.standard_data.payload()?);
        self.sysex_builder = self
            .sysex_builder
            .payload(self.discovery_data.iter().cloned());
        Ok(DiscoveryBorrowed(self.sysex_builder.build()?))
    }
}
