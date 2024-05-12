use crate::{
    detail::{common_properties, schema},
    system_common::{self, UMP_MESSAGE_TYPE},
};

pub const STATUS: u8 = 0xF2;

#[midi2_proc::generate_message(Via(system_common::SystemCommon), FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
struct SongPositionPointer {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(system_common::SystemCommonStatus<{STATUS}>)]
    status: (),
    #[property(common_properties::GroupProperty)]
    group: crate::ux::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u14,
        schema::Bytes<0x0, 0x7F, 0x7F>,
        schema::Ump<0x0000_7F7F, 0x0, 0x0, 0x0>,
    >)]
    position: crate::ux::u14,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Grouped, ux::*};
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = SongPositionPointer::new_arr();
        message.set_group(u4::new(0xA));
        message.set_position(u14::new(0x367D));
        assert_eq!(message, SongPositionPointer([0x1AF2_7D6C, 0x0, 0x0, 0x0]),);
    }
    #[test]
    fn setters_bytes() {
        let mut message = SongPositionPointer::new_arr_bytes();
        message.set_position(u14::new(0x367D));
        assert_eq!(message, SongPositionPointer([0xF2, 0x7D, 0x6C]),);
    }

    #[test]
    fn group() {
        assert_eq!(
            SongPositionPointer::try_from(&[0x1AF2_7D6C_u32][..])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn position() {
        assert_eq!(
            SongPositionPointer::try_from(&[0x1AF2_7D6C_u32][..])
                .unwrap()
                .position(),
            u14::new(0x367D),
        );
    }

    #[test]
    fn position_bytes() {
        assert_eq!(
            SongPositionPointer::try_from(&[0xF2_u8, 0x7D, 0x6C][..])
                .unwrap()
                .position(),
            u14::new(0x367D),
        );
    }
}
