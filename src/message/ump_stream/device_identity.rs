use crate::message::ump_stream::TYPE_CODE as UMP_STREAM_TYPE;
const STATUS: u32 = 0x2;

#[midi2_attr::generate_message()]
struct DeviceIdentity {
    ump_type:
        Property<NumericalConstant<UMP_STREAM_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x0C00_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x03FF_0000, 0x0, 0x0, 0x0>, ()>,
    device_manufacturer: Property<[u7; 3], UmpSchema<0x0, 0x007F_7F7F, 0x0, 0x0>, ()>,
    device_family: Property<u14, UmpSchema<0x0, 0x0, 0x7F7F_0000, 0x0>, ()>,
    device_family_model_number: Property<u14, UmpSchema<0x0, 0x0, 0x0000_7F7F, 0x0>, ()>,
    software_version: Property<[u7; 4], UmpSchema<0x0, 0x0, 0x0, 0x7F7F_7F7F>, ()>,
}

impl Property<[u7; 3], UmpSchema<0x0, 0x007F_7F7F, 0x0, 0x0>, ()> for Ump {
    fn get(data: &[<Ump as Buffer>::Data]) -> [u7; 3] {
        [data[1].septet(1), data[1].septet(2), data[1].septet(3)]
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: [u7; 3]) {
        data[1].set_septet(1, v[0]);
        data[1].set_septet(2, v[1]);
        data[1].set_septet(3, v[2]);
    }
}

impl Property<[u7; 4], UmpSchema<0x0, 0x0, 0x0, 0x7F7F_7F7F>, ()> for Ump {
    fn get(data: &[<Ump as Buffer>::Data]) -> [u7; 4] {
        [
            data[3].septet(0),
            data[3].septet(1),
            data[3].septet(2),
            data[3].septet(3),
        ]
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: [u7; 4]) {
        data[3].set_septet(0, v[0]);
        data[3].set_septet(1, v[1]);
        data[3].set_septet(2, v[2]);
        data[3].set_septet(3, v[3]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            DeviceIdentityMessage::builder()
                .device_manufacturer([u7::new(0x0F), u7::new(0x33), u7::new(0x28)])
                .device_family(u14::new(0xF4A))
                .device_family_model_number(u14::new(0x3818))
                .software_version([u7::new(0x43), u7::new(0x54), u7::new(0x32), u7::new(0x1)])
                .build(),
            Ok(DeviceIdentityMessage::Owned(DeviceIdentityOwned([
                0xF002_0000,
                0x000f_3328,
                0x4A1E_1870,
                0x4354_3201,
            ]))),
        );
    }

    #[test]
    fn device_manufacturer() {
        assert_eq!(
            DeviceIdentityMessage::from_data(
                &[0xF002_0000, 0x000f_3328, 0x4A1E_1870, 0x4354_3201,]
            )
            .unwrap()
            .device_manufacturer(),
            [u7::new(0x0F), u7::new(0x33), u7::new(0x28)],
        );
    }

    #[test]
    fn device_family() {
        assert_eq!(
            DeviceIdentityMessage::from_data(
                &[0xF002_0000, 0x000f_3328, 0x4A1E_1870, 0x4354_3201,]
            )
            .unwrap()
            .device_family(),
            u14::new(0xF4A),
        );
    }

    #[test]
    fn device_family_model_number() {
        assert_eq!(
            DeviceIdentityMessage::from_data(
                &[0xF002_0000, 0x000f_3328, 0x4A1E_1870, 0x4354_3201,]
            )
            .unwrap()
            .device_family_model_number(),
            u14::new(0x3818),
        );
    }

    #[test]
    fn software_version() {
        assert_eq!(
            DeviceIdentityMessage::from_data(
                &[0xF002_0000, 0x000f_3328, 0x4A1E_1870, 0x4354_3201,]
            )
            .unwrap()
            .software_version(),
            [u7::new(0x43), u7::new(0x54), u7::new(0x32), u7::new(0x1)],
        );
    }
}
