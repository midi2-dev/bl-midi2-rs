use crate::{
    message::{
        common_properties,
        flex_data::{self, UMP_MESSAGE_TYPE},
    },
    numeric_types::{u3, u4},
    util::{schema, BitOps},
};

const STATUS: u8 = 0x5;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct SetKeySignature {
    #[property(crate::message::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::message::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::GroupProperty)]
    group: crate::numeric_types::u4,
    #[property(flex_data::OptionalChannelProperty)]
    optional_channel: Option<crate::numeric_types::u4>,
    #[property(flex_data::FormatProperty<{flex_data::COMPLETE_FORMAT}>)]
    format: (),
    #[property(flex_data::BankProperty<{flex_data::SETUP_AND_PERFORMANCE_BANK}>)]
    bank: (),
    #[property(flex_data::StatusProperty<{STATUS}>)]
    status: (),
    #[property(flex_data::tonic::TonicProperty<schema::Ump<0x0, 0x0F00_0000, 0x0, 0x0>>)]
    tonic: flex_data::tonic::Tonic,
    #[property(SharpsFlatsProperty)]
    sharps_flats: SharpsFlats,
}

impl<B: crate::buffer::Ump> flex_data::FlexData<B> for SetKeySignature<B> {}

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

struct SharpsFlatsProperty;

impl<B: crate::buffer::Ump> crate::util::property::Property<B> for SharpsFlatsProperty {
    type Type = SharpsFlats;
}

impl<'a, B: crate::buffer::Ump> crate::util::property::ReadProperty<'a, B> for SharpsFlatsProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        use SharpsFlats::*;
        match u8::from(buffer.buffer().message()[1].nibble(0)) {
            v @ 0x0..=0x7 => Sharps(u3::new(v)),
            v @ 0x9..=0xF => Flats(u3::new(!(v - 1) & 0b0111)),
            0x8 => NonStandard,
            _ => unreachable!(),
        }
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::util::property::WriteProperty<B>
    for SharpsFlatsProperty
{
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        buffer.buffer_mut().message_mut()[1].set_nibble(
            0,
            match v {
                SharpsFlats::Sharps(v) => u4::from(v),
                SharpsFlats::Flats(v) => u4::new((!u8::from(v) & 0b0000_1111) + 1),
                SharpsFlats::NonStandard => u4::new(0x8),
            },
        );
    }
    fn validate(_: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Grouped;
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = SetKeySignature::new_arr();
        message.set_group(u4::new(0x4));
        message.set_tonic(flex_data::tonic::Tonic::D);
        message.set_sharps_flats(SharpsFlats::Sharps(u3::new(5)));
        assert_eq!(
            message,
            SetKeySignature([0x0, 0xD410_0005, 0x5400_0000, 0x0, 0x0,]),
        );
    }

    #[test]
    fn set_flats() {
        let mut message = SetKeySignature::new_arr();
        message.set_group(u4::new(0x4));
        message.set_tonic(flex_data::tonic::Tonic::D);
        message.set_sharps_flats(SharpsFlats::Flats(u3::new(5)));
        assert_eq!(
            message,
            SetKeySignature([0x0, 0xD410_0005, 0xB400_0000, 0x0, 0x0,]),
        );
    }

    #[test]
    fn builder_non_standard() {
        let mut message = SetKeySignature::new_arr();
        message.set_group(u4::new(0x4));
        message.set_tonic(flex_data::tonic::Tonic::NonStandard);
        message.set_sharps_flats(SharpsFlats::NonStandard);
        assert_eq!(
            message,
            SetKeySignature([0x0, 0xD410_0005, 0x8000_0000, 0x0, 0x0,]),
        );
    }

    #[test]
    fn builder_channel() {
        let mut message = SetKeySignature::new_arr();
        message.set_group(u4::new(0x4));
        message.set_tonic(flex_data::tonic::Tonic::NonStandard);
        message.set_sharps_flats(SharpsFlats::NonStandard);
        message.set_optional_channel(Some(u4::new(0xD)));
        assert_eq!(
            message,
            SetKeySignature([0x0, 0xD40D_0005, 0x8000_0000, 0x0, 0x0,]),
        );
    }

    #[test]
    fn tonic() {
        assert_eq!(
            SetKeySignature::try_from(&[0xD410_0005, 0x5400_0000][..])
                .unwrap()
                .tonic(),
            flex_data::tonic::Tonic::D,
        );
    }

    #[test]
    fn sharps_flats() {
        assert_eq!(
            SetKeySignature::try_from(&[0xD410_0005, 0x5400_0000][..])
                .unwrap()
                .sharps_flats(),
            SharpsFlats::Sharps(u3::new(5)),
        );
    }

    #[test]
    fn sharps_flats_with_flats() {
        assert_eq!(
            SetKeySignature::try_from(&[0xD410_0005, 0xB400_0000][..])
                .unwrap()
                .sharps_flats(),
            SharpsFlats::Flats(u3::new(5)),
        );
    }

    #[test]
    fn sharps_flats_non_standard() {
        assert_eq!(
            SetKeySignature::try_from(&[0xD410_0005, 0x8000_0000][..])
                .unwrap()
                .sharps_flats(),
            SharpsFlats::NonStandard,
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            SetKeySignature::try_from(&[0xD40D_0005, 0x8000_0000][..])
                .unwrap()
                .optional_channel(),
            Some(u4::new(0xD)),
        );
    }

    #[test]
    fn no_channel() {
        assert_eq!(
            SetKeySignature::try_from(&[0xD410_0005, 0x8000_0000][..])
                .unwrap()
                .optional_channel(),
            None,
        );
    }
}
