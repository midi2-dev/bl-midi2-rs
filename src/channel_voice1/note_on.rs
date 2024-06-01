use crate::{
    channel_voice1::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
};

// need to check out running statuses for full support
// of the MIDI 1.0 byte stream protocol

pub(crate) const STATUS: u8 = 0b1001;

/// MIDI 1.0 Channel Voice Note On Message
///
/// See the [module docs](crate::channel_voice1) for more info.
#[midi2_proc::generate_message(
    Via(crate::channel_voice1::ChannelVoice1),
    FixedSize,
    MinSizeUmp(1),
    MinSizeBytes(3)
)]
struct NoteOn {
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
    note: crate::ux::u7,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x00, 0x0, 0x7F>,
        schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>,
    >)]
    velocity: crate::ux::u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        traits::{Channeled, Grouped},
        ux::*,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = NoteOn::<[u32; 4]>::new();
        message.set_group(u4::new(0xD));
        message.set_channel(u4::new(0xE));
        message.set_note(u7::new(0x75));
        message.set_velocity(u7::new(0x3D));
        assert_eq!(message, NoteOn([0x2D9E_753D, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn setters_bytes() {
        let mut message = NoteOn::<[u8; 3]>::new();

        message.set_channel(u4::new(0xE));
        message.set_note(u7::new(0x75));
        message.set_velocity(u7::new(0x3D));

        assert_eq!(message, NoteOn([0x9E, 0x75, 0x3D]));
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOn::try_from(&[0x2D9E_753D_u32][..]).unwrap().group(),
            u4::new(0xD),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOn::try_from(&[0x2D9E_753D_u32][..]).unwrap().channel(),
            u4::new(0xE),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOn::try_from(&[0x2D9E_753D_u32][..]).unwrap().note(),
            u7::new(0x75),
        );
    }

    #[test]
    fn velocity() {
        assert_eq!(
            NoteOn::try_from(&[0x2D9E_753D_u32][..]).unwrap().velocity(),
            u7::new(0x3D),
        );
    }
}
