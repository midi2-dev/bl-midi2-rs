use crate::message::ump_stream::TYPE_CODE as UMP_STREAM_TYPE;
const STATUS: u32 = 0x10;

#[midi2_attr::generate_message()]
struct FunctionBlockDiscovery {
    ump_type:
        Property<NumericalConstant<UMP_STREAM_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x0C00_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x03FF_0000, 0x0, 0x0, 0x0>, ()>,
    function_block_number: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    requesting_function_block_info:
        Property<bool, UmpSchema<0b0000_0000_0000_0000_0000_0000_0000_0010, 0x0, 0x0, 0x0>, ()>,
    requesting_function_block_name:
        Property<bool, UmpSchema<0b0000_0000_0000_0000_0000_0000_0000_0001, 0x0, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            FunctionBlockDiscoveryMessage::builder()
                .function_block_number(u7::new(0x09))
                .requesting_function_block_info(true)
                .requesting_function_block_name(true)
                .build(),
            Ok(FunctionBlockDiscoveryMessage::Owned(
                FunctionBlockDiscoveryOwned([0xF010_0903, 0x0, 0x0, 0x0])
            ))
        );
    }

    #[test]
    fn function_block_number() {
        assert_eq!(
            FunctionBlockDiscoveryMessage::from_data(&[0xF010_0903])
                .unwrap()
                .function_block_number(),
            u7::new(0x09),
        );
    }

    #[test]
    fn requesting_function_block_info() {
        assert_eq!(
            FunctionBlockDiscoveryMessage::from_data(&[0xF010_0903])
                .unwrap()
                .requesting_function_block_info(),
            true
        );
    }

    #[test]
    fn requesting_function_block_name() {
        assert_eq!(
            FunctionBlockDiscoveryMessage::from_data(&[0xF010_0903])
                .unwrap()
                .requesting_function_block_name(),
            true
        );
    }
}
