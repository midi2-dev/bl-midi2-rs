use crate::{
    channel_voice2::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
    ux::{u4, u7},
};

pub(crate) const STATUS: u8 = 0b0011;

#[midi2_proc::generate_message(Via(crate::channel_voice2::ChannelVoice2), FixedSize, MinSizeUmp(2))]
struct AssignableController {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>>)]
    channel: u4,
    #[property(common_properties::GroupProperty)]
    group: u4,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>>)]
    bank: u7,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>>)]
    index: u7,
    #[property(common_properties::UmpSchemaProperty<u32, schema::Ump<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>>)]
    controller_data: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        use crate::traits::{Channeled, Grouped};

        let mut message = AssignableController::<[u32; 4]>::new();
        message.set_group(u4::new(0xC));
        message.set_channel(u4::new(0x8));
        message.set_bank(u7::new(0x51));
        message.set_index(u7::new(0x38));
        message.set_controller_data(0x3F3ADD42);
        assert_eq!(
            message,
            AssignableController([0x4C38_5138, 0x3F3ADD42, 0x0, 0x0]),
        );
    }

    #[test]
    fn group() {
        use crate::traits::Grouped;

        assert_eq!(
            AssignableController::try_from(&[0x4C38_5138, 0x3F3ADD42][..])
                .unwrap()
                .group(),
            u4::new(0xC),
        );
    }

    #[test]
    fn channel() {
        use crate::traits::Channeled;

        assert_eq!(
            AssignableController::try_from(&[0x4C38_5138, 0x3F3ADD42][..])
                .unwrap()
                .channel(),
            u4::new(0x8),
        );
    }

    #[test]
    pub fn bank() {
        assert_eq!(
            AssignableController::try_from(&[0x4C38_5138, 0x3F3ADD42][..])
                .unwrap()
                .bank(),
            u7::new(0x51),
        );
    }

    #[test]
    pub fn index() {
        assert_eq!(
            AssignableController::try_from(&[0x4C38_5138, 0x3F3ADD42][..])
                .unwrap()
                .index(),
            u7::new(0x38),
        );
    }

    #[test]
    pub fn controller_data() {
        assert_eq!(
            AssignableController::try_from(&[0x4C38_5138, 0x3F3ADD42][..])
                .unwrap()
                .controller_data(),
            0x3F3ADD42,
        );
    }
}
