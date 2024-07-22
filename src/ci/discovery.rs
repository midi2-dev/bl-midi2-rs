use crate::{ci, detail::property};

pub(crate) const STATUS: u8 = 0x70;

#[midi2_proc::generate_ci(
    SupportedVersion(version = 0x1, min_size = 31),
    SupportedVersion(version = 0x2, min_size = 32)
)]
struct DiscoveryQuery {
    #[property(ci::common_properties::UniversalSystemExclusiveByteProperty)]
    #[version(0x1)]
    universal_sysex_byte: (),
    #[property(ci::common_properties::DeviceIdToFunctionBlockProperty)]
    #[version(0x1)]
    device_id: (),
    #[property(ci::common_properties::UniversalSysexSubId1MidiCi)]
    #[version(0x1)]
    sysex_sub_id1_ci: (),
    #[property(ci::common_properties::UniversalSysexSubId2<STATUS>)]
    #[version(0x1)]
    sysex_sub_id2_ci: (),
    #[property(ci::common_properties::SourceProperty)]
    #[version(0x1)]
    source: ux::u28,
    #[property(ci::common_properties::BroadcastDestinationProperty)]
    #[version(0x1)]
    broadcast_destination: (),
    #[property(DeviceManufacturerProperty)]
    #[version(0x1)]
    device_manufacturer: [ux::u7; 3],
    #[property(DeviceFamilyProperty)]
    #[version(0x1)]
    device_family: ux::u14,
    #[property(DeviceModelNumberProperty)]
    #[version(0x1)]
    model_number: ux::u14,
    #[property(SoftwareVersionProperty)]
    #[version(0x1)]
    software_version: [ux::u7; 4],
    #[property(CiSupportProperty<3>)]
    #[version(0x1)]
    process_inquiry_supported: bool,
    #[property(CiSupportProperty<4>)]
    #[version(0x1)]
    property_exchange_supported: bool,
    #[property(CiSupportProperty<5>)]
    #[version(0x1)]
    profile_configuration_supported: bool,
    #[property(CiSupportProperty<6>)]
    #[version(0x1)]
    protocol_negotiation_supported: bool,
    #[property(MaxSysexSizeProperty)]
    #[version(0x1)]
    max_sysex_size: ux::u28,
    #[property(OutputPathIdProperty)]
    #[version(0x2)]
    output_path_id: ux::u7,
}

struct DeviceManufacturerProperty;

impl<B: crate::buffer::Bytes> property::Property<B> for DeviceManufacturerProperty {
    type Type = [ux::u7; 3];
}

impl<'a, B: crate::buffer::Bytes> property::ReadProperty<'a, B> for DeviceManufacturerProperty {
    fn read(buffer: &'a B) -> Self::Type {
        [
            ux::u7::new(buffer.buffer()[14]),
            ux::u7::new(buffer.buffer()[15]),
            ux::u7::new(buffer.buffer()[16]),
        ]
    }
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
}

impl<B: crate::buffer::Bytes + crate::buffer::BufferMut> property::WriteProperty<B>
    for DeviceManufacturerProperty
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[14] = v[0].into();
        buffer.buffer_mut()[15] = v[1].into();
        buffer.buffer_mut()[16] = v[2].into();
    }
}

struct DeviceFamilyProperty;

impl<B: crate::buffer::Bytes> property::Property<B> for DeviceFamilyProperty {
    type Type = ux::u14;
}

impl<'a, B: crate::buffer::Bytes> property::ReadProperty<'a, B> for DeviceFamilyProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::detail::Encode7Bit;
        ux::u14::from_u7s(&buffer.buffer()[17..19])
    }
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
}

impl<B: crate::buffer::Bytes + crate::buffer::BufferMut> property::WriteProperty<B>
    for DeviceFamilyProperty
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::detail::Encode7Bit;
        v.to_u7s(&mut buffer.buffer_mut()[17..19])
    }
}

struct DeviceModelNumberProperty;

impl<B: crate::buffer::Bytes> property::Property<B> for DeviceModelNumberProperty {
    type Type = ux::u14;
}

impl<'a, B: crate::buffer::Bytes> property::ReadProperty<'a, B> for DeviceModelNumberProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::detail::Encode7Bit;
        ux::u14::from_u7s(&buffer.buffer()[19..21])
    }
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
}

impl<B: crate::buffer::Bytes + crate::buffer::BufferMut> property::WriteProperty<B>
    for DeviceModelNumberProperty
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::detail::Encode7Bit;
        v.to_u7s(&mut buffer.buffer_mut()[19..21])
    }
}

struct SoftwareVersionProperty;

impl<B: crate::buffer::Bytes> property::Property<B> for SoftwareVersionProperty {
    type Type = [ux::u7; 4];
}

impl<'a, B: crate::buffer::Bytes> property::ReadProperty<'a, B> for SoftwareVersionProperty {
    fn read(buffer: &'a B) -> Self::Type {
        [
            ux::u7::new(buffer.buffer()[21]),
            ux::u7::new(buffer.buffer()[22]),
            ux::u7::new(buffer.buffer()[23]),
            ux::u7::new(buffer.buffer()[24]),
        ]
    }
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
}

impl<B: crate::buffer::Bytes + crate::buffer::BufferMut> property::WriteProperty<B>
    for SoftwareVersionProperty
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[21] = v[0].into();
        buffer.buffer_mut()[22] = v[1].into();
        buffer.buffer_mut()[23] = v[2].into();
        buffer.buffer_mut()[24] = v[3].into();
    }
}

struct CiSupportProperty<const BIT: usize>;

impl<const BIT: usize, B: crate::buffer::Bytes> property::Property<B> for CiSupportProperty<BIT> {
    type Type = bool;
}

impl<'a, const BIT: usize, B: crate::buffer::Bytes> property::ReadProperty<'a, B>
    for CiSupportProperty<BIT>
{
    fn read(buffer: &'a B) -> Self::Type {
        use crate::detail::BitOps;
        buffer.buffer()[25].bit(BIT)
    }
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
}

impl<const BIT: usize, B: crate::buffer::Bytes + crate::buffer::BufferMut>
    property::WriteProperty<B> for CiSupportProperty<BIT>
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::detail::BitOps;
        buffer.buffer_mut()[25].set_bit(BIT, v);
    }
}

struct MaxSysexSizeProperty;

impl<B: crate::buffer::Bytes> property::Property<B> for MaxSysexSizeProperty {
    type Type = ux::u28;
}

impl<'a, B: crate::buffer::Bytes> property::ReadProperty<'a, B> for MaxSysexSizeProperty {
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        use crate::detail::Encode7Bit;
        ux::u28::from_u7s(&buffer.buffer()[26..30])
    }
}

impl<B: crate::buffer::Bytes + crate::buffer::BufferMut> property::WriteProperty<B>
    for MaxSysexSizeProperty
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::detail::Encode7Bit;
        v.to_u7s(&mut buffer.buffer_mut()[26..30]);
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

struct OutputPathIdProperty;

impl<B: crate::buffer::Bytes> property::Property<B> for OutputPathIdProperty {
    type Type = ux::u7;
}

impl<'a, B: crate::buffer::Bytes> property::ReadProperty<'a, B> for OutputPathIdProperty {
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        ux::u7::new(buffer.buffer()[30])
    }
}

impl<B: crate::buffer::Bytes + crate::buffer::BufferMut> property::WriteProperty<B>
    for OutputPathIdProperty
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[30] = v.into();
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        message.set_source(ux::u28::new(0xB48D9D9));
        message.set_device_manufacturer([ux::u7::new(0x21), ux::u7::new(0x66), ux::u7::new(0x61)]);
        message.set_device_family(ux::u14::new(0x278A));
        message.set_model_number(ux::u14::new(0x2269));
        message.set_software_version([
            ux::u7::new(0x30),
            ux::u7::new(0x49),
            ux::u7::new(0xB),
            ux::u7::new(0x63),
        ]);
        message.set_process_inquiry_supported(true);
        message.set_property_exchange_supported(true);
        message.set_profile_configuration_supported(true);
        message.set_protocol_negotiation_supported(true);
        message.set_max_sysex_size(ux::u28::new(0xEF6EFE2));
        message.set_output_path_id(ux::u7::new(0x25));

        assert_eq!(
            message.data(),
            &[
                0xF0,
                0x7E,
                0x7F,
                0x0D,
                0x70,
                0x02,
                0x59,
                0x33,
                0x23,
                0x5A,
                0x7F,
                0x7F,
                0x7F,
                0x7F,
                0x21,
                0x66,
                0x61,
                0x0A,
                0x4F,
                0x69,
                0x44,
                0x30,
                0x49,
                0x0B,
                0x63,
                0b0001_1110,
                0x62,
                0x5F,
                0x5B,
                0x77,
                0x25,
                0xF7,
            ]
        );
    }

    #[test]
    fn new() {
        use crate::Data;

        let message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        assert_eq!(
            message.data(),
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x02, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xF7,
            ]
        );
    }

    #[test]
    fn try_new_with_buffer() {
        use crate::Data;

        let mut buffer = [0x0; 50];
        let message = DiscoveryQuery::<0x2, _>::try_new_with_buffer(&mut buffer[..]).unwrap();
        assert_eq!(
            message.data(),
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x02, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xF7,
            ]
        );
    }

    #[test]
    fn new_with_buffer_dirty() {
        use crate::Data;

        let mut buffer = (0..50).collect::<std::vec::Vec<u8>>();
        let message = DiscoveryQuery::<0x2, _>::new_with_buffer(&mut buffer);
        assert_eq!(
            message.data(),
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x02, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xF7,
            ]
        );
    }

    #[test]
    fn try_new() {
        use crate::Data;

        let message = DiscoveryQuery::<0x2, [u8; 32]>::try_new().expect("Buffer is large enough");
        assert_eq!(
            message.data(),
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x02, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xF7,
            ]
        );
    }

    #[test]
    fn try_new_v1() {
        use crate::Data;

        let message = DiscoveryQuery::<0x1, [u8; 31]>::try_new().expect("Buffer is large enough");
        assert_eq!(
            message.data(),
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x01, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0xF7,
            ]
        );
    }

    #[test]
    fn new_v1() {
        use crate::Data;

        let message = DiscoveryQuery::<0x1, std::vec::Vec<u8>>::new();
        assert_eq!(
            message.data(),
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x01, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0xF7,
            ]
        );
    }

    #[test]
    fn try_from_slice() {
        DiscoveryQuery::<0x2, _>::try_from(
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x02, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xF7,
            ][..],
        )
        .expect("Valid data");
    }

    #[test]
    fn rebuffer_from() {
        use crate::{Data, RebufferFrom};

        let borrowed = DiscoveryQuery::<0x2, _>::try_from(
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x02, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xF7,
            ][..],
        )
        .expect("Valid data");

        let owned = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::rebuffer_from(borrowed.clone());

        assert_eq!(borrowed.data(), owned.data());
    }

    #[test]
    fn try_rebuffer_from() {
        use crate::{Data, TryRebufferFrom};

        let borrowed = DiscoveryQuery::<0x2, _>::try_from(
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x02, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xF7,
            ][..],
        )
        .expect("Valid data");

        let owned = DiscoveryQuery::<0x2, [u8; 32]>::try_rebuffer_from(borrowed.clone())
            .expect("Valid data");

        assert_eq!(borrowed.data(), owned.data());
    }

    #[test]
    fn try_from_slice_v1() {
        DiscoveryQuery::<0x1, _>::try_from(
            &[
                0xF0, 0x7E, 0x7F, 0x0D, 0x70, 0x01, 0x0, 0x0, 0x0, 0x0, 0x7F, 0x7F, 0x7F, 0x7F,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0xF7,
            ][..],
        )
        .expect("Valid data");
    }

    #[test]
    fn source() {
        use crate::ci::Ci;
        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        message.set_source(ux::u28::new(0xEAEB42D));
        assert_eq!(message.source(), ux::u28::new(0xEAEB42D));
    }

    #[test]
    fn destination() {
        use crate::ci::Ci;
        let message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        assert_eq!(message.destination(), ux::u28::MAX);
    }

    #[test]
    fn device_id() {
        use crate::ci::{Ci, DeviceId};
        let message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        assert_eq!(message.device_id(), DeviceId::FunctionBlock);
    }

    #[test]
    fn device_manufactuter() {
        use crate::Data;

        let id = [ux::u7::new(0x47), ux::u7::new(0x13), ux::u7::new(0x01)];

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        message.set_device_manufacturer(id);

        assert_eq!(message.device_manufacturer(), id);
        assert_eq!(&message.data()[14..17], &[0x47, 0x13, 0x01]);
    }

    #[test]
    fn device_manufactuter_v1() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x1, std::vec::Vec<u8>>::new();
        let id = [ux::u7::new(0x47), ux::u7::new(0x13), ux::u7::new(0x01)];
        message.set_device_manufacturer(id);

        assert_eq!(message.device_manufacturer(), id);
        assert_eq!(&message.data()[14..17], &[0x47, 0x13, 0x01]);
    }

    #[test]
    fn device_family() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        let family = ux::u14::new(0x15FE);
        message.set_device_family(family);

        assert_eq!(message.device_family(), family);
        assert_eq!(&message.data()[17..19], &[0x7E, 0x2B]);
    }

    #[test]
    fn device_model_number() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        let model_number = ux::u14::new(0x6B8);
        message.set_model_number(model_number);

        assert_eq!(message.model_number(), model_number);
        assert_eq!(&message.data()[19..21], &[0x38, 0x0D]);
    }

    #[test]
    fn software_version() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        let version = [
            ux::u7::new(0x53),
            ux::u7::new(0x15),
            ux::u7::new(0x75),
            ux::u7::new(0x19),
        ];
        message.set_software_version(version);

        assert_eq!(message.software_version(), version);
        assert_eq!(&message.data()[21..25], &[0x53, 0x15, 0x75, 0x19]);
    }

    #[test]
    fn process_inquiry_supported() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        assert!(!message.process_inquiry_supported());

        message.set_process_inquiry_supported(true);
        assert!(message.process_inquiry_supported());

        assert_eq!(message.data()[25], 0b0001_0000);
    }

    #[test]
    fn profile_configuration_supported() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        assert!(!message.profile_configuration_supported());

        message.set_profile_configuration_supported(true);
        assert!(message.profile_configuration_supported());

        assert_eq!(message.data()[25], 0b0000_0100);
    }

    #[test]
    fn property_exchange_supported() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        assert!(!message.property_exchange_supported());

        message.set_property_exchange_supported(true);
        assert!(message.property_exchange_supported());

        assert_eq!(message.data()[25], 0b0000_1000);
    }

    #[test]
    fn protocol_negotiation_supported() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        assert!(!message.protocol_negotiation_supported());

        message.set_protocol_negotiation_supported(true);
        assert!(message.protocol_negotiation_supported());

        assert_eq!(message.data()[25], 0b0000_0010);
    }

    #[test]
    fn max_sysex_size() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        let value = ux::u28::new(0xB3ABD1C);
        message.set_max_sysex_size(value);

        assert_eq!(message.max_sysex_size(), value);
        assert_eq!(&message.data()[26..30], &[0x1C, 0x7A, 0x6A, 0x59]);
    }

    #[test]
    fn output_path_id() {
        use crate::Data;

        let mut message = DiscoveryQuery::<0x2, std::vec::Vec<u8>>::new();
        let value = ux::u7::new(0x08);
        message.set_output_path_id(value);
        assert_eq!(message.output_path_id(), value);
        assert_eq!(message.data()[30], u8::from(value));
    }
}
