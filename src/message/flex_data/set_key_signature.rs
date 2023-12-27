use crate::{
    message::flex_data::{
        tonic::Tonic, FlexData, SETUP_AND_PERFORMANCE_BANK, TYPE_CODE as FLEX_DATA_TYPE,
    },
    util::Truncate,
};

const STATUS: u32 = 0x5;

#[midi2_attr::generate_message(Grouped)]
struct SetTempo {
    ump_type:
        Property<NumericalConstant<FLEX_DATA_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x00C0_0000, 0x0, 0x0, 0x0>, ()>,
    bank: Property<
        NumericalConstant<SETUP_AND_PERFORMANCE_BANK>,
        UmpSchema<0x0000_FF00, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x0000_00FF, 0x0, 0x0, 0x0>, ()>,
    channel: Property<Option<u4>, UmpSchema<0x003F_0000, 0x0, 0x0, 0x0>, ()>,
    sharps_flats: Property<SharpsFlats, UmpSchema<0x0, 0xF000_0000, 0x0, 0x0>, ()>,
    tonic: Property<Tonic, UmpSchema<0x0, 0x0F00_0000, 0x0, 0x0>, ()>,
}

impl<'a> FlexData for SetTempoMessage<'a> {}
impl<'a> FlexData for SetTempoBorrowed<'a> {}
impl FlexData for SetTempoOwned {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharpsFlats {
    Flats(u3),
    Sharps(u3),
    NonStandard,
}

impl core::default::Default for SharpsFlats {
    /// Default is SharpsFlats::Sharps(0)
    fn default() -> Self {
        SharpsFlats::Sharps(ux::u3::default())
    }
}

impl Property<SharpsFlats, UmpSchema<0x0, 0xF000_0000, 0x0, 0x0>, ()> for Ump {
    fn get(data: &[<Ump as Buffer>::Data]) -> SharpsFlats {
        use SharpsFlats::*;
        match u8::from(data[1].nibble(0)) {
            v @ 0x0..=0x7 => Sharps(v.truncate()),
            // bug in the ux::u4 Not operator means we must go via u8 here
            v @ 0x9..=0xF => Flats((!(v - 1)).truncate()),
            0x8 => NonStandard,
            _ => panic!(),
        }
    }
    fn write(data: &mut [<Ump as Buffer>::Data], v: SharpsFlats) {
        data[1].set_nibble(
            0,
            match v {
                SharpsFlats::Sharps(v) => u4::from(v),
                // bug in the ux::u4 Not operator means we must go via u8 here
                SharpsFlats::Flats(v) => u4::new((!u8::from(v) & 0b0000_1111) + 1),
                SharpsFlats::NonStandard => u4::new(0x8),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            SetTempoMessage::builder()
                .group(u4::new(0x4))
                .tonic(Tonic::D)
                .sharps_flats(SharpsFlats::Sharps(u3::new(5)))
                .build(),
            Ok(SetTempoMessage::Owned(SetTempoOwned([
                0xD410_0005,
                0x5400_0000,
                0x0,
                0x0,
            ]))),
        );
    }

    #[test]
    fn builder_flats() {
        assert_eq!(
            SetTempoMessage::builder()
                .group(u4::new(0x4))
                .tonic(Tonic::D)
                .sharps_flats(SharpsFlats::Flats(u3::new(5)))
                .build(),
            Ok(SetTempoMessage::Owned(SetTempoOwned([
                0xD410_0005,
                0xB400_0000,
                0x0,
                0x0,
            ]))),
        );
    }

    #[test]
    fn builder_non_standard() {
        assert_eq!(
            SetTempoMessage::builder()
                .group(u4::new(0x4))
                .tonic(Tonic::NonStandard)
                .sharps_flats(SharpsFlats::NonStandard)
                .build(),
            Ok(SetTempoMessage::Owned(SetTempoOwned([
                0xD410_0005,
                0x8000_0000,
                0x0,
                0x0,
            ]))),
        );
    }

    #[test]
    fn builder_channel() {
        assert_eq!(
            SetTempoMessage::builder()
                .group(u4::new(0x4))
                .channel(Some(u4::new(0xD)))
                .tonic(Tonic::NonStandard)
                .sharps_flats(SharpsFlats::NonStandard)
                .build(),
            Ok(SetTempoMessage::Owned(SetTempoOwned([
                0xD40D_0005,
                0x8000_0000,
                0x0,
                0x0,
            ]))),
        );
    }

    #[test]
    fn tonic() {
        assert_eq!(
            SetTempoMessage::from_data(&[0xD410_0005, 0x5400_0000])
                .unwrap()
                .tonic(),
            Tonic::D,
        );
    }

    #[test]
    fn sharps_flats() {
        assert_eq!(
            SetTempoMessage::from_data(&[0xD410_0005, 0x5400_0000])
                .unwrap()
                .sharps_flats(),
            SharpsFlats::Sharps(u3::new(5)),
        );
    }

    #[test]
    fn sharps_flats_with_flats() {
        assert_eq!(
            SetTempoMessage::from_data(&[0xD410_0005, 0xB400_0000])
                .unwrap()
                .sharps_flats(),
            SharpsFlats::Flats(u3::new(5)),
        );
    }

    #[test]
    fn sharps_flats_non_standard() {
        assert_eq!(
            SetTempoMessage::from_data(&[0xD410_0005, 0x8000_0000])
                .unwrap()
                .sharps_flats(),
            SharpsFlats::NonStandard,
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            SetTempoMessage::from_data(&[0xD40D_0005, 0x8000_0000])
                .unwrap()
                .channel(),
            Some(u4::new(0xD)),
        );
    }

    #[test]
    fn no_channel() {
        assert_eq!(
            SetTempoMessage::from_data(&[0xD410_0005, 0x8000_0000])
                .unwrap()
                .channel(),
            None,
        );
    }
}
