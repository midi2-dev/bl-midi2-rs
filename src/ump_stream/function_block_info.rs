use crate::{
    detail::{common_properties, property, schema},
    ump_stream,
    ump_stream::UMP_MESSAGE_TYPE,
    ux::{u4, u7},
};

pub(crate) const STATUS: u16 = 0x11;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct FunctionBlockInfo {
    #[property(crate::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0b0000_0000_0000_0000_1000_0000_0000_0000, 0x0, 0x0, 0x0>>)]
    active: bool,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>>)]
    function_block_number: u7,
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x0, 0x0F00_0000, 0x0, 0x0>>)]
    first_group: u4,
    #[property(common_properties::UmpSchemaProperty<u8, schema::Ump<0x0, 0x00FF_0000, 0x0, 0x0>>)]
    number_of_groups_spanned: u8,
    #[property(common_properties::UmpSchemaProperty<u8, schema::Ump<0x0, 0x0000_FF00, 0x0, 0x0>>)]
    midi_ci_version: u8,
    #[property(common_properties::UmpSchemaProperty<u8, schema::Ump<0x0, 0x0000_00FF, 0x0, 0x0>>)]
    max_number_of_midi_ci_streams: u8,
    #[property(UiHintProperty)]
    ui_hint: UiHint,
    #[property(Midi1PortProperty)]
    midi1_port: Option<Midi1Port>,
    #[property(DirectionProperty)]
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiHint {
    Undeclared,
    Sender,
    Receiver,
    SenderReciever,
}

struct UiHintProperty;

impl<B: crate::buffer::Ump> property::Property<B> for UiHintProperty {
    type Type = UiHint;
}

impl<'a, B: crate::buffer::Ump> property::ReadProperty<'a, B> for UiHintProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        use crate::detail::BitOps;
        use UiHint::*;

        match u8::from(buffer.buffer().message()[0].crumb(13)) {
            0b00 => Undeclared,
            0b01 => Receiver,
            0b10 => Sender,
            0b11 => SenderReciever,
            _ => unreachable!(),
        }
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> property::WriteProperty<B>
    for UiHintProperty
{
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::detail::BitOps;
        use crate::numeric_types::u2;
        use UiHint::*;

        buffer.buffer_mut().message_mut()[0].set_crumb(
            13,
            match v {
                Undeclared => u2::new(0b00),
                Receiver => u2::new(0b01),
                Sender => u2::new(0b10),
                SenderReciever => u2::new(0b11),
            },
        );
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

impl core::default::Default for UiHint {
    /// Default value is UiHint::Undeclared
    fn default() -> Self {
        UiHint::Undeclared
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Midi1Port {
    RestrictBandwidth,
    DontRestrictBandwidth,
}

struct Midi1PortProperty;

impl<B: crate::buffer::Ump> property::Property<B> for Midi1PortProperty {
    type Type = Option<Midi1Port>;
}

impl<'a, B: crate::buffer::Ump> property::ReadProperty<'a, B> for Midi1PortProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        use crate::detail::BitOps;
        use Midi1Port::*;

        match u8::from(buffer.buffer().message()[0].crumb(14)) {
            0b00 => None,
            0b01 => Some(DontRestrictBandwidth),
            0b10 => Some(RestrictBandwidth),
            _ => panic!(),
        }
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::detail::BitOps;

        match u8::from(buffer.buffer().message()[0].crumb(14)) {
            0b00 => Ok(()),
            0b01 => Ok(()),
            0b10 => Ok(()),
            _ => Err(crate::error::Error::InvalidData(
                "Couldn't interpret midi1 port field",
            )),
        }
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> property::WriteProperty<B>
    for Midi1PortProperty
{
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::detail::BitOps;
        use crate::numeric_types::u2;
        use Midi1Port::*;

        buffer.buffer_mut().message_mut()[0].set_crumb(
            14,
            match v {
                None => u2::new(0b00),
                Some(DontRestrictBandwidth) => u2::new(0b01),
                Some(RestrictBandwidth) => u2::new(0b10),
            },
        );
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Input,
    Output,
    Bidirectional,
}

struct DirectionProperty;

impl<B: crate::buffer::Ump> property::Property<B> for DirectionProperty {
    type Type = Direction;
}

impl<'a, B: crate::buffer::Ump> property::ReadProperty<'a, B> for DirectionProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        use crate::detail::BitOps;
        use Direction::*;

        match u8::from(buffer.buffer().message()[0].crumb(15)) {
            0b01 => Input,
            0b10 => Output,
            0b11 => Bidirectional,
            _ => panic!(),
        }
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::detail::BitOps;

        match u8::from(buffer.buffer().message()[0].crumb(15)) {
            0b00 => Err(crate::error::Error::InvalidData(
                "Couldn't interpret direction field",
            )),
            0b01 => Ok(()),
            0b10 => Ok(()),
            0b11 => Ok(()),
            _ => unreachable!(),
        }
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> property::WriteProperty<B>
    for DirectionProperty
{
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::detail::BitOps;
        use crate::numeric_types::u2;
        use Direction::*;

        buffer.buffer_mut().message_mut()[0].set_crumb(
            15,
            match v {
                Input => u2::new(0b01),
                Output => u2::new(0b10),
                Bidirectional => u2::new(0b11),
            },
        );
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

impl core::default::Default for Direction {
    /// Default value is Direction::Bidirectional
    fn default() -> Self {
        Direction::Bidirectional
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = FunctionBlockInfo::new_arr();
        message.set_active(true);
        message.set_function_block_number(u7::new(0x11));
        message.set_first_group(u4::new(0xD));
        message.set_number_of_groups_spanned(0x8);
        message.set_midi_ci_version(0x1);
        message.set_max_number_of_midi_ci_streams(0x20);
        message.set_ui_hint(UiHint::SenderReciever);
        message.set_midi1_port(Some(Midi1Port::DontRestrictBandwidth));
        message.set_direction(Direction::Output);

        assert_eq!(
            message,
            FunctionBlockInfo([0x0, 0xF011_9136, 0x0D08_0120, 0x0, 0x0,])
        )
    }

    #[test]
    fn active() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .active(),
            true
        );
    }

    #[test]
    fn function_block_number() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .function_block_number(),
            u7::new(0x11),
        );
    }

    #[test]
    fn first_group() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .first_group(),
            u4::new(0xD),
        );
    }

    #[test]
    fn number_of_groups_spanned() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .number_of_groups_spanned(),
            0x8,
        );
    }

    #[test]
    fn midi_ci_version() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .midi_ci_version(),
            0x1,
        );
    }

    #[test]
    fn max_number_of_midi_ci_streams() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .max_number_of_midi_ci_streams(),
            0x20,
        );
    }

    #[test]
    fn ui_hint() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .ui_hint(),
            UiHint::SenderReciever,
        );
    }

    #[test]
    fn midi1_port() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .midi1_port(),
            Some(Midi1Port::DontRestrictBandwidth),
        );
    }

    #[test]
    fn direction() {
        assert_eq!(
            FunctionBlockInfo::try_from(&[0xF011_9136, 0x0D08_0120][..])
                .unwrap()
                .direction(),
            Direction::Output,
        );
    }
}
