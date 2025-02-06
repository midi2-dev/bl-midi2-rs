use crate::{
    channel_voice2::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
    ux::{u4, u7},
};

pub(crate) const STATUS: u8 = 0b0100;

/// MIDI 2.0 Channel Voice Relative Registered Controller Message
///
/// See the [module docs](crate::channel_voice2) for more info.
#[midi2_proc::generate_message(Via(crate::channel_voice2::ChannelVoice2), FixedSize, MinSizeUmp(2))]
struct RelativeRegisteredController {
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
    fn builder() {
        use crate::traits::{Channeled, Grouped};

        let mut message = RelativeRegisteredController::<[u32; 4]>::new();
        message.set_group(u4::new(0x1));
        message.set_channel(u4::new(0xE));
        message.set_bank(u7::new(0x45));
        message.set_index(u7::new(0x02));
        message.set_controller_data(0xAF525908);

        assert_eq!(
            message,
            RelativeRegisteredController([0x414E_4502, 0xAF525908, 0x0, 0x0,]),
        );
    }

    #[test]
    pub fn bank() {
        assert_eq!(
            RelativeRegisteredController::try_from(&[0x414E_4502, 0xAF525908][..])
                .unwrap()
                .bank(),
            u7::new(0x45),
        );
    }

    #[test]
    pub fn index() {
        assert_eq!(
            RelativeRegisteredController::try_from(&[0x414E_4502, 0xAF525908][..])
                .unwrap()
                .index(),
            u7::new(0x02),
        );
    }

    #[test]
    pub fn controller_data() {
        assert_eq!(
            RelativeRegisteredController::try_from(&[0x414E_4502, 0xAF525908][..])
                .unwrap()
                .controller_data(),
            0xAF525908,
        );
    }
}
