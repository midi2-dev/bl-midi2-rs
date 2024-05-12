use crate::{
    detail::{common_properties, schema},
    system_common::{self, UMP_MESSAGE_TYPE},
};

pub const STATUS: u8 = 0xF3;

#[midi2_proc::generate_message(
    Via(system_common::SystemCommon),
    FixedSize,
    MinSizeUmp(1),
    MinSizeBytes(2)
)]
struct SongSelect {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(system_common::SystemCommonStatus<{STATUS}>)]
    status: (),
    #[property(common_properties::GroupProperty)]
    group: crate::ux::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x0, 0x7F, 0x0>,
        schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>,
    >)]
    song: crate::ux::u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Grouped, ux::*};
    use pretty_assertions::assert_eq;

    #[test]
    fn setter() {
        let mut message = SongSelect::<[u32; 4]>::new();
        message.set_group(u4::new(0xA));
        message.set_song(u7::new(0x4F));
        assert_eq!(message, SongSelect([0x1AF3_4F00, 0x0, 0x0, 0x0]),);
    }

    #[test]
    fn setters_bytes() {
        let mut message = SongSelect::<[u8; 3]>::new();
        message.set_song(u7::new(0x4F));
        assert_eq!(message, SongSelect([0xF3, 0x4F, 0x0]),);
    }

    #[test]
    fn group() {
        assert_eq!(
            SongSelect::try_from(&[0x1AF3_4F00_u32][..])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn song() {
        assert_eq!(
            SongSelect::try_from(&[0x1AF3_4F00_u32][..]).unwrap().song(),
            u7::new(0x4F),
        );
    }

    #[test]
    fn song_bytes() {
        assert_eq!(
            SongSelect::try_from(&[0xF3_u8, 0x4F][..]).unwrap().song(),
            u7::new(0x4F),
        )
    }
}
