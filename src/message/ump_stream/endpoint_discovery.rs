use crate::message::ump_stream::TYPE_CODE as UMP_STREAM_TYPE;
const STATUS: u32 = 0x0;

#[midi2_attr::generate_message()]
struct EndpointDiscovery {
    ump_type:
        Property<NumericalConstant<UMP_STREAM_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x0C00_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x03FF_0000, 0x0, 0x0, 0x0>, ()>,
    ump_version_major: Property<u8, UmpSchema<0x0000_FF00, 0x0, 0x0, 0x0>, ()>,
    ump_version_minor: Property<u8, UmpSchema<0x0000_00FF, 0x0, 0x0, 0x0>, ()>,
    request_endpoint_info:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0001, 0x0, 0x0>, ()>,
    request_device_identity:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0010, 0x0, 0x0>, ()>,
    request_endpoint_name:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0100, 0x0, 0x0>, ()>,
    request_product_instance_id:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0000_0000_1000, 0x0, 0x0>, ()>,
    request_stream_configuration:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0000_0001_0000, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            EndpointDiscoveryMessage::builder()
                .ump_version_major(0x1)
                .ump_version_minor(0x1)
                .request_endpoint_info(true)
                .request_device_identity(true)
                .request_endpoint_name(true)
                .request_product_instance_id(true)
                .request_stream_configuration(true)
                .build(),
            Ok(EndpointDiscoveryMessage::Owned(EndpointDiscoveryOwned([
                0xF000_0101,
                0x0000_001F,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn ump_version_major() {
        assert_eq!(
            EndpointDiscoveryMessage::from_data(&[0xF000_0101, 0x0000_001F])
                .unwrap()
                .ump_version_major(),
            0x1,
        );
    }

    #[test]
    fn ump_version_minor() {
        assert_eq!(
            EndpointDiscoveryMessage::from_data(&[0xF000_0101, 0x0000_001F])
                .unwrap()
                .ump_version_minor(),
            0x1,
        );
    }

    #[test]
    fn request_endpoint_info() {
        assert_eq!(
            EndpointDiscoveryMessage::from_data(&[0xF000_0101, 0x0000_001F])
                .unwrap()
                .request_endpoint_info(),
            true,
        );
    }

    #[test]
    fn request_device_identity() {
        assert_eq!(
            EndpointDiscoveryMessage::from_data(&[0xF000_0101, 0x0000_001F])
                .unwrap()
                .request_device_identity(),
            true,
        );
    }

    #[test]
    fn request_endpoint_name() {
        assert_eq!(
            EndpointDiscoveryMessage::from_data(&[0xF000_0101, 0x0000_001F])
                .unwrap()
                .request_endpoint_name(),
            true,
        );
    }

    #[test]
    fn request_product_instance_id() {
        assert_eq!(
            EndpointDiscoveryMessage::from_data(&[0xF000_0101, 0x0000_001F])
                .unwrap()
                .request_product_instance_id(),
            true,
        );
    }

    #[test]
    fn request_stream_configuration() {
        assert_eq!(
            EndpointDiscoveryMessage::from_data(&[0xF000_0101, 0x0000_001F])
                .unwrap()
                .request_stream_configuration(),
            true,
        );
    }
}
