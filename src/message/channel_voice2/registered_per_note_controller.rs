use crate::{
    message::{
        channel_voice2::{controller, UMP_MESSAGE_TYPE},
        common_properties,
    },
    numeric_types::{u4, u7},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b0000;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct RegisteredPerNoteController {
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
    #[property(controller::ControllerProperty)]
    controller: controller::Controller,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        use crate::traits::{Channeled, Grouped};

        let mut message = RegisteredPerNoteController::new_arr();
        message.set_group(u4::new(0x4));
        message.set_channel(u4::new(0x5));
        message.set_note(u7::new(0x6C));
        message.set_controller(controller::Controller::Volume(0xE1E35E92));

        assert_eq!(
            message,
            RegisteredPerNoteController([0x0, 0x4405_6C07, 0xE1E35E92, 0x0, 0x0,]),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            RegisteredPerNoteController::try_from(&[0x4405_6C07, 0xE1E35E92][..])
                .unwrap()
                .note(),
            u7::new(0x6C),
        );
    }

    #[test]
    fn controller() {
        assert_eq!(
            RegisteredPerNoteController::try_from(&[0x4405_6C07, 0xE1E35E92][..])
                .unwrap()
                .controller(),
            controller::Controller::Volume(0xE1E35E92),
        );
    }
}
