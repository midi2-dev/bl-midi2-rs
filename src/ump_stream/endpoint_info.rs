use crate::{
    detail::{common_properties, schema},
    ump_stream,
    ump_stream::UMP_MESSAGE_TYPE,
    ux::u7,
};

pub(crate) const STATUS: u16 = 0x01;

#[midi2_proc::generate_message(Via(ump_stream::UmpStream), FixedSize, MinSizeUmp(2))]
struct EndpointInfo {
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
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b1000_0000_0000_0000_0000_0000_0000_0000, 0x0, 0x0>>)]
    static_function_blocks: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0010_0000_0000, 0x0, 0x0>>)]
    supports_midi2_protocol: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0001_0000_0000, 0x0, 0x0>>)]
    supports_midi1_protocol: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0010, 0x0, 0x0>>)]
    supports_receiving_jr_timestamps: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0001, 0x0, 0x0>>)]
    supports_sending_jr_timestamps: bool,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0, 0x7F00_0000, 0x0, 0x0>>)]
    number_of_function_blocks: u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = EndpointInfo::new_arr();
        message.set_ump_version_major(0x1);
        message.set_ump_version_minor(0x1);
        message.set_static_function_blocks(true);
        message.set_number_of_function_blocks(u7::new(0x20));
        message.set_supports_midi2_protocol(true);
        message.set_supports_midi1_protocol(true);
        message.set_supports_sending_jr_timestamps(true);
        message.set_supports_receiving_jr_timestamps(true);

        assert_eq!(
            message,
            EndpointInfo([
                0xF001_0101,
                0b1010_0000_0000_0000_0000_0011_0000_0011,
                0x0,
                0x0
            ])
        );
    }

    #[test]
    fn ump_version_major() {
        assert_eq!(
            EndpointInfo::try_from(&[0xF001_0101, 0b1010_0000_0000_0000_0000_0011_0000_0011,][..])
                .unwrap()
                .ump_version_major(),
            0x1,
        );
    }

    #[test]
    fn ump_version_minor() {
        assert_eq!(
            EndpointInfo::try_from(&[0xF001_0101, 0b1010_0000_0000_0000_0000_0011_0000_0011,][..])
                .unwrap()
                .ump_version_minor(),
            0x1,
        );
    }

    #[test]
    fn static_function_blocks() {
        assert_eq!(
            EndpointInfo::try_from(&[0xF001_0101, 0b1010_0000_0000_0000_0000_0011_0000_0011,][..])
                .unwrap()
                .static_function_blocks(),
            true,
        );
    }

    #[test]
    fn number_of_function_blocks() {
        assert_eq!(
            EndpointInfo::try_from(&[0xF001_0101, 0b1010_0000_0000_0000_0000_0011_0000_0011,][..])
                .unwrap()
                .number_of_function_blocks(),
            u7::new(0x20),
        );
    }

    #[test]
    fn supports_midi2_protocol() {
        assert_eq!(
            EndpointInfo::try_from(&[0xF001_0101, 0b1010_0000_0000_0000_0000_0011_0000_0011,][..])
                .unwrap()
                .supports_midi2_protocol(),
            true,
        );
    }

    #[test]
    fn supports_midi1_protocol() {
        assert_eq!(
            EndpointInfo::try_from(&[0xF001_0101, 0b1010_0000_0000_0000_0000_0011_0000_0011,][..])
                .unwrap()
                .supports_midi1_protocol(),
            true,
        );
    }

    #[test]
    fn supports_sending_jr_timestamps() {
        assert_eq!(
            EndpointInfo::try_from(&[0xF001_0101, 0b1010_0000_0000_0000_0000_0011_0000_0011,][..])
                .unwrap()
                .supports_sending_jr_timestamps(),
            true,
        );
    }

    #[test]
    fn supports_receiving_jr_timestamps() {
        assert_eq!(
            EndpointInfo::try_from(&[0xF001_0101, 0b1010_0000_0000_0000_0000_0011_0000_0011,][..])
                .unwrap()
                .supports_receiving_jr_timestamps(),
            true,
        );
    }
}
