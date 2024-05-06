use crate::{
    detail::{common_properties, schema},
    system_common::{self, UMP_MESSAGE_TYPE},
};

pub const STATUS: u8 = 0xF1;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(3))]
struct TimeCode {
    #[property(crate::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(system_common::SystemCommonStatus<{STATUS}>)]
    status: (),
    #[property(common_properties::GroupProperty)]
    group: crate::numeric_types::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::numeric_types::u7,
        schema::Bytes<0x0, 0x7F, 0x0>,
        schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>,
    >)]
    time_code: crate::numeric_types::u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{numeric_types::*, traits::Grouped};
    use pretty_assertions::assert_eq;

    #[test]
    fn setter() {
        let mut message = TimeCode::new_arr();
        message.set_group(u4::new(0x5));
        message.set_time_code(u7::new(0x5F));
        assert_eq!(message, TimeCode([0x0, 0x15F1_5F00, 0x0, 0x0, 0x0]),);
    }

    #[test]
    fn setters_bytes() {
        let mut message = TimeCode::new_arr_bytes();
        message.set_time_code(u7::new(0x5F));
        assert_eq!(message, TimeCode([0xF1, 0x5F, 0x0,]),);
    }

    #[test]
    fn group() {
        assert_eq!(
            TimeCode::try_from(&[0x15F1_5F00_u32][..]).unwrap().group(),
            u4::new(0x5),
        );
    }

    #[test]
    fn time_code() {
        assert_eq!(
            TimeCode::try_from(&[0x15F1_5F00_u32][..])
                .unwrap()
                .time_code(),
            u7::new(0x5F),
        );
    }
}
