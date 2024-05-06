use crate::{
    detail::{common_properties, schema},
    ump_stream,
    ump_stream::UMP_MESSAGE_TYPE,
};

pub(crate) const STATUS: u16 = 0x0;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct EndpointDiscovery {
    #[property(crate::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
    #[property(common_properties::UmpSchemaProperty<u8, schema::Ump<0x0000_FF00, 0x0, 0x0, 0x0>>)]
    ump_version_major: u8,
    #[property(common_properties::UmpSchemaProperty<u8, schema::Ump<0x0000_00FF, 0x0, 0x0, 0x0>>)]
    ump_version_minor: u8,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0001, 0x0, 0x0>>)]
    request_endpoint_info: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0010, 0x0, 0x0>>)]
    request_device_identity: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0100, 0x0, 0x0>>)]
    request_endpoint_name: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0000_0000_1000, 0x0, 0x0>>)]
    request_product_instance_id: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0000_0001_0000, 0x0, 0x0>>)]
    request_stream_configuration: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = EndpointDiscovery::new_arr();
        message.set_ump_version_major(0x1);
        message.set_ump_version_minor(0x1);
        message.set_request_endpoint_info(true);
        message.set_request_device_identity(true);
        message.set_request_endpoint_name(true);
        message.set_request_product_instance_id(true);
        message.set_request_stream_configuration(true);
        assert_eq!(
            message,
            EndpointDiscovery([0x0, 0xF000_0101, 0x0000_001F, 0x0, 0x0]),
        );
    }

    #[test]
    fn ump_version_major() {
        assert_eq!(
            EndpointDiscovery::try_from(&[0xF000_0101, 0x0000_001F][..])
                .unwrap()
                .ump_version_major(),
            0x1,
        );
    }

    #[test]
    fn ump_version_minor() {
        assert_eq!(
            EndpointDiscovery::try_from(&[0xF000_0101, 0x0000_001F][..])
                .unwrap()
                .ump_version_minor(),
            0x1,
        );
    }

    #[test]
    fn request_endpoint_info() {
        assert_eq!(
            EndpointDiscovery::try_from(&[0xF000_0101, 0x0000_001F][..])
                .unwrap()
                .request_endpoint_info(),
            true,
        );
    }

    #[test]
    fn request_device_identity() {
        assert_eq!(
            EndpointDiscovery::try_from(&[0xF000_0101, 0x0000_001F][..])
                .unwrap()
                .request_device_identity(),
            true,
        );
    }

    #[test]
    fn request_endpoint_name() {
        assert_eq!(
            EndpointDiscovery::try_from(&[0xF000_0101, 0x0000_001F][..])
                .unwrap()
                .request_endpoint_name(),
            true,
        );
    }

    #[test]
    fn request_product_instance_id() {
        assert_eq!(
            EndpointDiscovery::try_from(&[0xF000_0101, 0x0000_001F][..])
                .unwrap()
                .request_product_instance_id(),
            true,
        );
    }

    #[test]
    fn request_stream_configuration() {
        assert_eq!(
            EndpointDiscovery::try_from(&[0xF000_0101, 0x0000_001F][..])
                .unwrap()
                .request_stream_configuration(),
            true,
        );
    }
}
