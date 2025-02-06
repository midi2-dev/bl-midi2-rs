use crate::{
    channel_voice1::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
};

pub(crate) const STATUS: u8 = 0b1000;

/// MIDI 1.0 Channel Voice Note Off Message
///
/// See the [module docs](crate::channel_voice1) for more info.
#[midi2_proc::generate_message(
    Via(crate::channel_voice1::ChannelVoice1),
    FixedSize,
    MinSizeUmp(1),
    MinSizeBytes(3)
)]
struct NoteOff {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::ChannelProperty)]
    channel: crate::ux::u4,
    #[property(common_properties::GroupProperty)]
    group: crate::ux::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x00, 0x7F, 0x0>,
        schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>,
    >)]
    note_number: crate::ux::u7,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x00, 0x0, 0x7F>,
        schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>,
    >)]
    velocity: crate::ux::u7,
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
//     note_number: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
//     velocity: Property<u7, UmpSchema<0x0000_007F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x0, 0x7F>>,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        traits::{Channeled, Grouped},
        ux::*,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = NoteOff::<[u32; 4]>::new();
        message.set_group(u4::new(0x1));
        message.set_channel(u4::new(0xA));
        message.set_note_number(u7::new(0x68));
        message.set_velocity(u7::new(0x1B));
        assert_eq!(message, NoteOff([0x218A_681B, 0x0, 0x0, 0x0]));
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
    fn note_number() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..])
                .unwrap()
                .note_number(),
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
