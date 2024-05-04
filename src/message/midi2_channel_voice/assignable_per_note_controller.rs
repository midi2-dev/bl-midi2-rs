use crate::{
    message::{common_properties, midi2_channel_voice::UMP_MESSAGE_TYPE},
    numeric_types::{u4, u7},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b0001;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct AssignablePerNoteController {
    #[property(crate::message::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::message::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>>)]
    channel: u4,
    #[property(common_properties::GroupProperty)]
    group: u4,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>>)]
    note: u7,
    #[property(common_properties::UmpSchemaProperty<u8, schema::Ump<0x0000_00FF, 0x0, 0x0, 0x0>>)]
    index: u8,
    #[property(common_properties::UmpSchemaProperty<u32, schema::Ump<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>>)]
    controller_data: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        use crate::traits::{Channeled, Grouped};

        let mut message = AssignablePerNoteController::new_arr();
        message.set_group(u4::new(0x2));
        message.set_channel(u4::new(0x4));
        message.set_note(u7::new(0x6F));
        message.set_index(0xB1);
        message.set_controller_data(0x46105EE5);

        assert_eq!(
            message,
            AssignablePerNoteController([0x0, 0x4214_6FB1, 0x46105EE5, 0x0, 0x0,]),
        );
    }

    #[test]
    fn group() {
        use crate::traits::Grouped;
        assert_eq!(
            AssignablePerNoteController::try_from(&[0x4214_6FB1, 0x46105EE5][..])
                .unwrap()
                .group(),
            u4::new(0x2),
        );
    }

    #[test]
    fn channel() {
        use crate::traits::Channeled;
        assert_eq!(
            AssignablePerNoteController::try_from(&[0x4214_6FB1, 0x46105EE5][..])
                .unwrap()
                .channel(),
            u4::new(0x4),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            AssignablePerNoteController::try_from(&[0x4214_6FB1, 0x46105EE5][..])
                .unwrap()
                .note(),
            u7::new(0x6F),
        );
    }

    #[test]
    fn index() {
        assert_eq!(
            AssignablePerNoteController::try_from(&[0x4214_6FB1, 0x46105EE5][..])
                .unwrap()
                .index(),
            0xB1,
        );
    }

    #[test]
    fn controller_data() {
        assert_eq!(
            AssignablePerNoteController::try_from(&[0x4214_6FB1, 0x46105EE5][..])
                .unwrap()
                .controller_data(),
            0x46105EE5,
        );
    }
}
