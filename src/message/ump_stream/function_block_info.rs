use crate::message::ump_stream::TYPE_CODE as UMP_STREAM_TYPE;
const STATUS: u32 = 0x11;

#[midi2_proc::generate_message()]
struct FunctionBlockInfo {
    ump_type:
        Property<NumericalConstant<UMP_STREAM_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x0C00_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x03FF_0000, 0x0, 0x0, 0x0>, ()>,
    active: Property<bool, UmpSchema<0b0000_0000_0000_0000_1000_0000_0000_0000, 0x0, 0x0, 0x0>, ()>,
    function_block_number: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    first_group: Property<u4, UmpSchema<0x0, 0x0F00_0000, 0x0, 0x0>, ()>,
    number_of_groups_spanned: Property<u8, UmpSchema<0x0, 0x00FF_0000, 0x0, 0x0>, ()>,
    midi_ci_version: Property<u8, UmpSchema<0x0, 0x0000_FF00, 0x0, 0x0>, ()>,
    max_number_of_midi_ci_streams: Property<u8, UmpSchema<0x0, 0x0000_00FF, 0x0, 0x0>, ()>,
    ui_hint:
        Property<UiHint, UmpSchema<0b0000_0000_0000_0000_0000_0000_0011_0000, 0x0, 0x0, 0x0>, ()>,
    midi1_port: Property<
        Option<Midi1Port>,
        UmpSchema<0b0000_0000_0000_0000_0000_0000_0000_1100, 0x0, 0x0, 0x0>,
        (),
    >,
    direction: Property<
        Direction,
        UmpSchema<0b0000_0000_0000_0000_0000_0000_0000_0011, 0x0, 0x0, 0x0>,
        (),
    >,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiHint {
    Undeclared,
    Sender,
    Receiver,
    SenderReciever,
}

impl<BytesSchema: Schema>
    Property<
        UiHint,
        UmpSchema<0b0000_0000_0000_0000_0000_0000_0011_0000, 0x0, 0x0, 0x0>,
        BytesSchema,
    > for Ump
{
    fn get(data: &[<Ump as Buffer>::Data]) -> UiHint {
        use UiHint::*;
        match u8::from(data[0].crumb(13)) {
            0b00 => Undeclared,
            0b01 => Receiver,
            0b10 => Sender,
            0b11 => SenderReciever,
            _ => unreachable!(),
        }
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: UiHint) {
        use UiHint::*;
        data[0].set_crumb(
            13,
            match v {
                Undeclared => u2::new(0b00),
                Receiver => u2::new(0b01),
                Sender => u2::new(0b10),
                SenderReciever => u2::new(0b11),
            },
        );
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

impl<BytesSchema: Schema>
    Property<
        Option<Midi1Port>,
        UmpSchema<0b0000_0000_0000_0000_0000_0000_0000_1100, 0x0, 0x0, 0x0>,
        BytesSchema,
    > for Ump
{
    fn get(data: &[<Ump as Buffer>::Data]) -> Option<Midi1Port> {
        use Midi1Port::*;
        match u8::from(data[0].crumb(14)) {
            0b00 => None,
            0b01 => Some(DontRestrictBandwidth),
            0b10 => Some(RestrictBandwidth),
            _ => panic!(),
        }
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: Option<Midi1Port>) {
        use Midi1Port::*;
        data[0].set_crumb(
            14,
            match v {
                None => u2::new(0b00),
                Some(DontRestrictBandwidth) => u2::new(0b01),
                Some(RestrictBandwidth) => u2::new(0b10),
            },
        );
    }
    fn validate(data: &[<Self as Buffer>::Data]) -> Result<()> {
        match u8::from(data[0].crumb(14)) {
            0b00 => Ok(()),
            0b01 => Ok(()),
            0b10 => Ok(()),
            0b11 => Err(Error::InvalidData),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Input,
    Output,
    Bidirectional,
}

impl<BytesSchema: Schema>
    Property<
        Direction,
        UmpSchema<0b0000_0000_0000_0000_0000_0000_0000_0011, 0x0, 0x0, 0x0>,
        BytesSchema,
    > for Ump
{
    fn get(data: &[<Ump as Buffer>::Data]) -> Direction {
        use Direction::*;
        match u8::from(data[0].crumb(15)) {
            0b01 => Input,
            0b10 => Output,
            0b11 => Bidirectional,
            _ => panic!(),
        }
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: Direction) {
        use Direction::*;
        data[0].set_crumb(
            15,
            match v {
                Input => u2::new(0b01),
                Output => u2::new(0b10),
                Bidirectional => u2::new(0b11),
            },
        );
    }
    fn validate(data: &[<Self as Buffer>::Data]) -> Result<()> {
        match u8::from(data[0].crumb(15)) {
            0b00 => Err(Error::InvalidData),
            0b01 => Ok(()),
            0b10 => Ok(()),
            0b11 => Ok(()),
            _ => unreachable!(),
        }
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
        assert_eq!(
            FunctionBlockInfoMessage::builder()
                .active(true)
                .function_block_number(u7::new(0x11))
                .first_group(u4::new(0xD))
                .number_of_groups_spanned(0x8)
                .midi_ci_version(0x1)
                .max_number_of_midi_ci_streams(0x20)
                .ui_hint(UiHint::SenderReciever)
                .midi1_port(Some(Midi1Port::DontRestrictBandwidth))
                .direction(Direction::Output)
                .build(),
            Ok(FunctionBlockInfoMessage::Owned(FunctionBlockInfoOwned([
                0xF011_9136,
                0x0D08_0120,
                0x0,
                0x0,
            ])))
        )
    }

    #[test]
    fn active() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .active(),
            true
        );
    }

    #[test]
    fn function_block_number() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .function_block_number(),
            u7::new(0x11),
        );
    }

    #[test]
    fn first_group() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .first_group(),
            u4::new(0xD),
        );
    }

    #[test]
    fn number_of_groups_spanned() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .number_of_groups_spanned(),
            0x8,
        );
    }

    #[test]
    fn midi_ci_version() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .midi_ci_version(),
            0x1,
        );
    }

    #[test]
    fn max_number_of_midi_ci_streams() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .max_number_of_midi_ci_streams(),
            0x20,
        );
    }

    #[test]
    fn ui_hint() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .ui_hint(),
            UiHint::SenderReciever,
        );
    }

    #[test]
    fn midi1_port() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .midi1_port(),
            Some(Midi1Port::DontRestrictBandwidth),
        );
    }

    #[test]
    fn direction() {
        assert_eq!(
            FunctionBlockInfoMessage::from_data(&[0xF011_9136, 0x0D08_0120])
                .unwrap()
                .direction(),
            Direction::Output,
        );
    }
}
