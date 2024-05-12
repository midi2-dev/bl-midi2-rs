use crate::{
    detail::{common_properties, schema},
    ump_stream,
    ump_stream::UMP_MESSAGE_TYPE,
    ux::{u14, u7},
};

pub(crate) const STATUS: u16 = 0x2;

#[midi2_proc::generate_message(Via(ump_stream::UmpStream), FixedSize, MinSizeUmp(4))]
struct DeviceIdentity {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
    #[property(common_properties::UmpSchemaProperty<[u7; 3], schema::Ump<0x0, 0x007F_7F7F, 0x0, 0x0>>)]
    device_manufacturer: [u7; 3],
    #[property(common_properties::UmpSchemaProperty<u14, schema::Ump<0x0, 0x0, 0x7F7F_0000, 0x0>>)]
    device_family: u14,
    #[property(common_properties::UmpSchemaProperty<u14, schema::Ump<0x0, 0x0, 0x0000_7F7F, 0x0>>)]
    device_family_model_number: u14,
    #[property(common_properties::UmpSchemaProperty<[u7; 4], schema::Ump<0x0, 0x0, 0x0, 0x7F7F_7F7F>>)]
    software_version: [u7; 4],
}

impl schema::UmpSchemaRepr<schema::Ump<0x0, 0x007F_7F7F, 0x0, 0x0>> for [crate::ux::u7; 3] {
    fn write(buffer: &mut [u32], value: Self) {
        use crate::detail::BitOps;
        buffer[1].set_septet(1, value[0]);
        buffer[1].set_septet(2, value[1]);
        buffer[1].set_septet(3, value[2]);
    }
    fn read(buffer: &[u32]) -> Self {
        use crate::detail::BitOps;
        [
            buffer[1].septet(1),
            buffer[1].septet(2),
            buffer[1].septet(3),
        ]
    }
}

impl schema::UmpSchemaRepr<schema::Ump<0x0, 0x0, 0x0, 0x7F7F_7F7F>> for [crate::ux::u7; 4] {
    fn write(buffer: &mut [u32], value: Self) {
        use crate::detail::BitOps;
        buffer[3].set_septet(0, value[0]);
        buffer[3].set_septet(1, value[1]);
        buffer[3].set_septet(2, value[2]);
        buffer[3].set_septet(3, value[3]);
    }
    fn read(buffer: &[u32]) -> Self {
        use crate::detail::BitOps;
        [
            buffer[3].septet(0),
            buffer[3].septet(1),
            buffer[3].septet(2),
            buffer[3].septet(3),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = DeviceIdentity::<[u32; 4]>::new();
        message.set_device_manufacturer([u7::new(0x0F), u7::new(0x33), u7::new(0x28)]);
        message.set_device_family(u14::new(0xF4A));
        message.set_device_family_model_number(u14::new(0x3818));
        message.set_software_version([u7::new(0x43), u7::new(0x54), u7::new(0x32), u7::new(0x1)]);
        assert_eq!(
            message,
            DeviceIdentity([0xF002_0000, 0x000f_3328, 0x4A1E_1870, 0x4354_3201,]),
        );
    }

    #[test]
    fn device_manufacturer() {
        assert_eq!(
            DeviceIdentity::try_from(&[0xF002_0000, 0x000F_3328, 0x4A1E_1870, 0x4354_3201][..])
                .unwrap()
                .device_manufacturer(),
            [u7::new(0x0F), u7::new(0x33), u7::new(0x28)],
        );
    }

    #[test]
    fn device_family() {
        assert_eq!(
            DeviceIdentity::try_from(&[0xF002_0000, 0x000F_3328, 0x4A1E_1870, 0x4354_3201][..])
                .unwrap()
                .device_family(),
            u14::new(0xF4A),
        );
    }

    #[test]
    fn device_family_model_number() {
        assert_eq!(
            DeviceIdentity::try_from(&[0xF002_0000, 0x000F_3328, 0x4A1E_1870, 0x4354_3201][..])
                .unwrap()
                .device_family_model_number(),
            u14::new(0x3818),
        );
    }

    #[test]
    fn software_version() {
        assert_eq!(
            DeviceIdentity::try_from(&[0xF002_0000, 0x000F_3328, 0x4A1E_1870, 0x4354_3201][..])
                .unwrap()
                .software_version(),
            [u7::new(0x43), u7::new(0x54), u7::new(0x32), u7::new(0x1)],
        );
    }
}
