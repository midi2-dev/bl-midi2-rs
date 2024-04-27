use crate::{
    message::{common_properties, midi1_channel_voice::UMP_MESSAGE_TYPE},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b1100;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
struct ProgramChange {
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
    program: crate::numeric_types::u7,
}

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
        let mut message = ProgramChange::new_arr();
        message.set_group(u4::new(0x4));
        message.set_channel(u4::new(0x7));
        message.set_program(u7::new(0x63));
        assert_eq!(message, ProgramChange([0x24C7_6300]));
    }
}
