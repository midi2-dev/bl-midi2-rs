use crate::message::ump_stream::TYPE_CODE as UMP_STREAM_TYPE;
const STATUS: u32 = 0x01;

#[midi2_attr::generate_message()]
struct EndpointInfo {
    ump_type:
        Property<NumericalConstant<UMP_STREAM_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x0C00_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x03FF_0000, 0x0, 0x0, 0x0>, ()>,
    ump_version_major: Property<u8, UmpSchema<0x0000_FF00, 0x0, 0x0, 0x0>, ()>,
    ump_version_minor: Property<u8, UmpSchema<0x0000_00FF, 0x0, 0x0, 0x0>, ()>,
    static_function_blocks:
        Property<bool, UmpSchema<0x0, 0b1000_0000_0000_0000_0000_0000_0000_0000, 0x0, 0x0>, ()>,
    number_of_function_blocks: Property<u7, UmpSchema<0x0, 0x7F00_0000, 0x0, 0x0>, ()>,
    supports_midi2_protocol:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0010_0000_0000, 0x0, 0x0>, ()>,
    supports_midi1_protocol:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0001_0000_0000, 0x0, 0x0>, ()>,
    supports_receiving_jr_timestamps:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0010, 0x0, 0x0>, ()>,
    supports_sending_jr_timestamps:
        Property<bool, UmpSchema<0x0, 0b0000_0000_0000_0000_0000_0000_0000_0001, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            EndpointInfoMessage::builder()
                .ump_version_major(0x1)
                .ump_version_minor(0x1)
                .static_function_blocks(true)
                .number_of_function_blocks(u7::new(0x20))
                .supports_midi2_protocol(true)
                .supports_midi1_protocol(true)
                .supports_sending_jr_timestamps(true)
                .supports_receiving_jr_timestamps(true)
                .build(),
            Ok(EndpointInfoMessage::Owned(EndpointInfoOwned([
                0xF001_0101,
                0b1010_0000_0000_0000_0000_0011_0000_0011,
                0x0,
                0x0
            ])))
        );
    }
}
