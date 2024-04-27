use crate::{
    message::{common_properties, midi1_channel_voice::UMP_MESSAGE_TYPE},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b1000;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(3))]
struct NoteOff {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::ChannelProperty)]
    channel: crate::numeric_types::u4,
    #[property(common_properties::GroupProperty)]
    group: crate::numeric_types::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::numeric_types::u7,
        schema::Bytes<0x00, 0x7F, 0x0>,
        schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>,
    >)]
    note: crate::numeric_types::u7,
    #[property(common_properties::HybridSchemaProperty<
        crate::numeric_types::u7,
        schema::Bytes<0x00, 0x0, 0x7F>,
        schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>,
    >)]
    velocity: crate::numeric_types::u7,
}

// #[midi2_proc::generate_message(Grouped, Channeled)]
// struct NoteOff {
//     ump_type: Property<
//         NumericalConstant<MIDI1_CHANNEL_VOICE_TYPE>,
//         UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
//         (),
//     >,
//     status: Property<
//         NumericalConstant<OP_CODE>,
//         UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//         BytesSchema<0xF0, 0x0, 0x0>,
//     >,
//     channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, BytesSchema<0x0F, 0x0, 0x0>>,
//     note: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
//     velocity: Property<u7, UmpSchema<0x0000_007F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x0, 0x7F>>,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        numeric_types::*,
        traits::{Channeled, Grouped},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = NoteOff::new_arr();
        message.set_group(u4::new(0x1));
        message.set_channel(u4::new(0xA));
        message.set_note(u7::new(0x68));
        message.set_velocity(u7::new(0x1B));
        assert_eq!(message, NoteOff([0x218A_681B]));
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..]).unwrap().group(),
            u4::new(0x1),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..]).unwrap().channel(),
            u4::new(0xA),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..]).unwrap().note(),
            u7::new(0x68),
        );
    }

    #[test]
    fn velocity() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..])
                .unwrap()
                .velocity(),
            u7::new(0x1B),
        );
    }
}
