use crate::{
    detail::{common_properties, schema},
    ump_stream,
    ump_stream::UMP_MESSAGE_TYPE,
};

pub(crate) const STATUS: u16 = 0x10;

#[midi2_proc::generate_message(Via(ump_stream::UmpStream), FixedSize, MinSizeUmp(1))]
struct FunctionBlockDiscovery {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),

    #[property(common_properties::UmpSchemaProperty<u8, schema::Ump<0x0000_FF00, 0x0, 0x0, 0x0>>)]
    function_block_number: u8,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0b0000_0000_0000_0000_0000_0000_0000_0010, 0x0, 0x0, 0x0>>)]
    requesting_function_block_info: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0b0000_0000_0000_0000_0000_0000_0000_0001, 0x0, 0x0, 0x0>>)]
    requesting_function_block_name: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = FunctionBlockDiscovery::new_arr();
        message.set_function_block_number(0x09);
        message.set_requesting_function_block_info(true);
        message.set_requesting_function_block_name(true);
        assert_eq!(
            message,
            FunctionBlockDiscovery([0xF010_0903, 0x0, 0x0, 0x0])
        );
    }

    #[test]
    fn function_block_number() {
        assert_eq!(
            FunctionBlockDiscovery::try_from(&[0xF010_0903][..])
                .unwrap()
                .function_block_number(),
            0x09,
        );
    }

    #[test]
    fn requesting_function_block_info() {
        assert_eq!(
            FunctionBlockDiscovery::try_from(&[0xF010_0903][..])
                .unwrap()
                .requesting_function_block_info(),
            true
        );
    }

    #[test]
    fn requesting_function_block_name() {
        assert_eq!(
            FunctionBlockDiscovery::try_from(&[0xF010_0903][..])
                .unwrap()
                .requesting_function_block_name(),
            true
        );
    }
}
