use crate::{
    channel_voice2::UMP_MESSAGE_TYPE,
    detail::{common_properties, property, schema},
    ux::{u14, u4, u7},
};

pub(crate) const STATUS: u8 = 0b1100;

/// MIDI 2.0 Channel Voice Program Change Message
///
/// See the [module docs](crate::channel_voice2) for more info.
#[midi2_proc::generate_message(Via(crate::channel_voice2::ChannelVoice2), FixedSize, MinSizeUmp(2))]
struct ProgramChange {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>>)]
    channel: u4,
    #[property(common_properties::GroupProperty)]
    group: u4,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0, 0x7F00_0000, 0x0, 0x0>>)]
    program: u7,
    #[property(BankProperty)]
    bank: Option<u14>,
}

struct BankProperty;

impl<B: crate::buffer::Ump> property::Property<B> for BankProperty {
    type Type = Option<u14>;
}

impl<'a, B: crate::buffer::Ump> property::ReadProperty<'a, B> for BankProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::detail::{BitOps, Encode7Bit};

        let data = buffer.buffer();
        if data[0].bit(31) {
            Some(u14::from_u7s(&[data[1].octet(2), data[1].octet(3)]))
        } else {
            None
        }
    }
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> property::WriteProperty<B> for BankProperty {
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::detail::{BitOps, Encode7Bit};

        let data = buffer.buffer_mut();
        match v {
            Some(v) => {
                let mut u7s = [u7::default(); 2];
                v.to_u7s(&mut u7s);
                data[1].set_octet(2, u7s[0].into());
                data[1].set_octet(3, u7s[1].into());
                data[0].set_bit(31, true);
            }
            None => {
                data[0].set_bit(31, false);
                data[1].set_word(1, 0x0);
            }
        }
    }
    fn default() -> Self::Type {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        use crate::traits::{Channeled, Grouped};

        let mut message = ProgramChange::<[u32; 4]>::new();
        message.set_group(u4::new(0xF));
        message.set_channel(u4::new(0xE));
        message.set_program(u7::new(0x75));
        message.set_bank(Some(u14::new(0x1F5E)));

        assert_eq!(message, ProgramChange([0x4FCE_0001, 0x7500_5E3E, 0x0, 0x0]),);
    }

    #[test]
    fn builder_no_bank() {
        use crate::traits::{Channeled, Grouped};

        let mut message = ProgramChange::<[u32; 4]>::new();
        message.set_group(u4::new(0xF));
        message.set_channel(u4::new(0xE));
        message.set_program(u7::new(0x75));

        assert_eq!(message, ProgramChange([0x4FCE_0000, 0x7500_0000, 0x0, 0x0]),);
    }

    #[test]
    fn program() {
        assert_eq!(
            ProgramChange::try_from(&[0x4FCE_0001, 0x7500_5E3E][..])
                .unwrap()
                .program(),
            u7::new(0x75),
        )
    }

    #[test]
    fn bank() {
        assert_eq!(
            ProgramChange::try_from(&[0x4FCE_0001, 0x7500_5E3E][..])
                .unwrap()
                .bank(),
            Some(u14::new(0x1F5E)),
        )
    }

    #[test]
    fn no_bank() {
        assert_eq!(
            ProgramChange::try_from(&[0x4FCE_0000, 0x7500_0000][..])
                .unwrap()
                .bank(),
            None,
        )
    }

    #[test]
    fn rebuffer_mut_slice_to_slice() {
        use crate::RebufferInto;
        let mut buffer = [0x0; 4];
        let mut mut_slice_message = ProgramChange::try_new_with_buffer(&mut buffer[..]).unwrap();
        mut_slice_message.set_program(u7::new(0x4F));
        let slice_message: ProgramChange<&[u32]> = mut_slice_message.rebuffer_into();
        assert_eq!(slice_message.data(), &[0x40C0_0000, 0x4F00_0000][..]);
    }
}
