use crate::{
    detail::{common_properties, schema, BitOps},
    error::Error,
    flex_data::{self, UMP_MESSAGE_TYPE},
    result::Result,
    ux::u4,
};

const STATUS: u8 = 0x6;

#[midi2_proc::generate_message(Via(crate::flex_data::FlexData), FixedSize, MinSizeUmp(4))]
struct SetChordName {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::GroupProperty)]
    group: crate::ux::u4,
    #[property(flex_data::OptionalChannelProperty)]
    optional_channel: Option<crate::ux::u4>,
    #[property(flex_data::FormatProperty<{flex_data::COMPLETE_FORMAT}>)]
    format: (),
    #[property(flex_data::BankProperty<{flex_data::SETUP_AND_PERFORMANCE_BANK}>)]
    bank: (),
    #[property(flex_data::StatusProperty<{STATUS}>)]
    status: (),
    #[property(SharpsFlatsProperty<schema::Ump<0x0, 0xF000_0000, 0x0, 0x0>>)]
    tonic_sharps_flats: SharpsFlats,
    #[property(flex_data::tonic::TonicProperty<schema::Ump<0x0, 0x0F00_0000, 0x0, 0x0>>)]
    tonic: flex_data::tonic::Tonic,
    #[property(ChordTypeProperty<schema::Ump<0x0, 0x00FF_0000, 0x0, 0x0>>)]
    chord_type: ChordType,
    #[property(AlterationProperty<schema::Ump<0x0, 0x0000_FF00, 0x0, 0x0>>)]
    chord_alteration1: Option<Alteration>,
    #[property(AlterationProperty<schema::Ump<0x0, 0x0000_00FF, 0x0, 0x0>>)]
    chord_alteration2: Option<Alteration>,
    #[property(AlterationProperty<schema::Ump<0x0, 0x0, 0xFF00_0000, 0x0>>)]
    chord_alteration3: Option<Alteration>,
    #[property(AlterationProperty<schema::Ump<0x0, 0x0, 0x00FF_0000, 0x0>>)]
    chord_alteration4: Option<Alteration>,
    #[property(SharpsFlatsProperty<schema::Ump<0x0, 0x0, 0x0, 0xF000_0000>>)]
    bass_sharps_flats: SharpsFlats,
    #[property(flex_data::tonic::TonicProperty<schema::Ump<0x0, 0x0, 0x0, 0x0F00_0000>>)]
    bass_note: flex_data::tonic::Tonic,
    #[property(ChordTypeProperty<schema::Ump<0x0, 0x0, 0x0, 0x00FF_0000>>)]
    bass_chord_type: ChordType,
    #[property(AlterationProperty<schema::Ump<0x0, 0x0, 0x0, 0x0000_FF00>>)]
    bass_alteration1: Option<Alteration>,
    #[property(AlterationProperty<schema::Ump<0x0, 0x0, 0x0, 0x0000_00FF>>)]
    bass_alteration2: Option<Alteration>,
}

impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for SetChordName<B> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharpsFlats {
    DoubleSharp,
    Sharp,
    Natural,
    Flat,
    DoubleFlat,
}

struct SharpsFlatsProperty<S: schema::UmpSchema>(S);

impl<B: crate::buffer::Ump, S: schema::UmpSchema> crate::detail::property::Property<B>
    for SharpsFlatsProperty<S>
{
    type Type = SharpsFlats;
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B>
    for SharpsFlatsProperty<schema::Ump<0x0, 0x0, 0x0, 0xF000_0000>>
{
    fn read(buffer: &'a B) -> Self::Type {
        SharpsFlats::from_nibble(buffer.buffer()[3].nibble(0)).unwrap()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        SharpsFlats::from_nibble(buffer.buffer()[3].nibble(0))?;
        Ok(())
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for SharpsFlatsProperty<schema::Ump<0x0, 0x0, 0x0, 0xF000_0000>>
{
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[3].set_nibble(0, v.into_nibble());
    }
    fn validate(_: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B>
    for SharpsFlatsProperty<schema::Ump<0x0, 0xF000_0000, 0x0, 0x0>>
{
    fn read(buffer: &'a B) -> Self::Type {
        SharpsFlats::from_nibble(buffer.buffer()[1].nibble(0)).unwrap()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        SharpsFlats::from_nibble(buffer.buffer()[1].nibble(0))?;
        Ok(())
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for SharpsFlatsProperty<schema::Ump<0x0, 0xF000_0000, 0x0, 0x0>>
{
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[1].set_nibble(0, v.into_nibble());
    }
    fn validate(_: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

impl SharpsFlats {
    fn from_nibble(nibble: u4) -> Result<SharpsFlats> {
        use SharpsFlats::*;
        match u8::from(nibble) {
            0x2 => Ok(DoubleSharp),
            0x1 => Ok(Sharp),
            0x0 => Ok(Natural),
            0xF => Ok(Flat),
            0xE => Ok(DoubleFlat),
            _ => Err(Error::InvalidData(
                "Couldn't interpret Sharps / Flats field",
            )),
        }
    }
    fn into_nibble(self) -> u4 {
        use SharpsFlats::*;
        u4::new(match self {
            DoubleSharp => 0x2,
            Sharp => 0x1,
            Natural => 0x0,
            Flat => 0xF,
            DoubleFlat => 0xE,
        })
    }
}

impl core::default::Default for SharpsFlats {
    /// Default value is [SharpsFlats::Natural]
    fn default() -> Self {
        SharpsFlats::Natural
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordType {
    ClearChord,
    Major,
    Major6th,
    Major7th,
    Major9th,
    Major11th,
    Major13th,
    Minor,
    Minor6th,
    Minor7th,
    Minor9th,
    Minor11th,
    Minor13th,
    Dominant,
    Dominant9th,
    Dominant11th,
    Dominant13th,
    Augmented,
    Augmented7th,
    Diminished,
    Diminished7th,
    HalfDiminished,
    MajorMinor,
    Pedal,
    Power,
    Suspended2nd,
    Suspended4th,
}

struct ChordTypeProperty<S: schema::UmpSchema>(S);

impl<B: crate::buffer::Ump, S: schema::UmpSchema> crate::detail::property::Property<B>
    for ChordTypeProperty<S>
{
    type Type = ChordType;
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B>
    for ChordTypeProperty<schema::Ump<0x0, 0x00FF_0000, 0x0, 0x0>>
{
    fn validate(buffer: &B) -> crate::result::Result<()> {
        ChordType::from_octet(buffer.buffer()[1].octet(1))?;
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        ChordType::from_octet(buffer.buffer()[1].octet(1)).unwrap()
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for ChordTypeProperty<schema::Ump<0x0, 0x00FF_0000, 0x0, 0x0>>
{
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[1].set_octet(1, v.into_octet());
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B>
    for ChordTypeProperty<schema::Ump<0x0, 0x0, 0x0, 0x00FF_0000>>
{
    fn validate(buffer: &B) -> crate::result::Result<()> {
        ChordType::from_octet(buffer.buffer()[3].octet(1))?;
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        ChordType::from_octet(buffer.buffer()[3].octet(1)).unwrap()
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for ChordTypeProperty<schema::Ump<0x0, 0x0, 0x0, 0x00FF_0000>>
{
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[3].set_octet(1, v.into_octet());
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

impl ChordType {
    fn from_octet(octet: u8) -> Result<Self> {
        use ChordType::*;
        match octet {
            0x00 => Ok(ClearChord),
            0x01 => Ok(Major),
            0x02 => Ok(Major6th),
            0x03 => Ok(Major7th),
            0x04 => Ok(Major9th),
            0x05 => Ok(Major11th),
            0x06 => Ok(Major13th),
            0x07 => Ok(Minor),
            0x08 => Ok(Minor6th),
            0x09 => Ok(Minor7th),
            0x0A => Ok(Minor9th),
            0x0B => Ok(Minor11th),
            0x0C => Ok(Minor13th),
            0x0D => Ok(Dominant),
            0x0E => Ok(Dominant9th),
            0x0F => Ok(Dominant11th),
            0x10 => Ok(Dominant13th),
            0x11 => Ok(Augmented),
            0x12 => Ok(Augmented7th),
            0x13 => Ok(Diminished),
            0x14 => Ok(Diminished7th),
            0x15 => Ok(HalfDiminished),
            0x16 => Ok(MajorMinor),
            0x17 => Ok(Pedal),
            0x18 => Ok(Power),
            0x19 => Ok(Suspended2nd),
            0x1A => Ok(Suspended4th),
            _ => Err(Error::InvalidData("Couldn't interpret Chord field")),
        }
    }

    fn into_octet(self) -> u8 {
        use ChordType::*;
        match self {
            ClearChord => 0x00,
            Major => 0x01,
            Major6th => 0x02,
            Major7th => 0x03,
            Major9th => 0x04,
            Major11th => 0x05,
            Major13th => 0x06,
            Minor => 0x07,
            Minor6th => 0x08,
            Minor7th => 0x09,
            Minor9th => 0x0A,
            Minor11th => 0x0B,
            Minor13th => 0x0C,
            Dominant => 0x0D,
            Dominant9th => 0x0E,
            Dominant11th => 0x0F,
            Dominant13th => 0x10,
            Augmented => 0x11,
            Augmented7th => 0x12,
            Diminished => 0x13,
            Diminished7th => 0x14,
            HalfDiminished => 0x15,
            MajorMinor => 0x16,
            Pedal => 0x17,
            Power => 0x18,
            Suspended2nd => 0x19,
            Suspended4th => 0x1A,
        }
    }
}

impl core::default::Default for ChordType {
    /// Default value is [ChordType::ClearChord]
    fn default() -> Self {
        ChordType::ClearChord
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alteration {
    Add(u4),
    Subtract(u4),
    Raise(u4),
    Lower(u4),
}

struct AlterationProperty<S: schema::UmpSchema>(S);

impl<B: crate::buffer::Ump, S: schema::UmpSchema> crate::detail::property::Property<B>
    for AlterationProperty<S>
{
    type Type = Option<Alteration>;
}

macro_rules! alteration_property_impl {
    ($ump1:expr,$ump2:expr,$ump3:expr,$ump4:expr,$buffer_index:expr,$octet_index:expr) => {
        impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B>
            for AlterationProperty<schema::Ump<$ump1, $ump2, $ump3, $ump4>>
        {
            fn validate(buffer: &B) -> crate::result::Result<()> {
                alteration_from_octet(buffer.buffer()[$buffer_index].octet($octet_index))?;
                Ok(())
            }
            fn read(buffer: &'a B) -> Self::Type {
                alteration_from_octet(buffer.buffer()[$buffer_index].octet($octet_index)).unwrap()
            }
        }

        impl<B: crate::buffer::Ump + crate::buffer::BufferMut>
            crate::detail::property::WriteProperty<B>
            for AlterationProperty<schema::Ump<$ump1, $ump2, $ump3, $ump4>>
        {
            fn validate(_v: &Self::Type) -> crate::result::Result<()> {
                Ok(())
            }
            fn write(buffer: &mut B, v: Self::Type) {
                buffer.buffer_mut()[$buffer_index]
                    .set_octet($octet_index, alteration_into_octet(v));
            }
            fn default() -> Self::Type {
                Default::default()
            }
        }
    };
}

alteration_property_impl!(0x0, 0x0000_FF00, 0x0, 0x0, 1, 2);
alteration_property_impl!(0x0, 0x0000_00FF, 0x0, 0x0, 1, 3);
alteration_property_impl!(0x0, 0x0, 0xFF00_0000, 0x0, 2, 0);
alteration_property_impl!(0x0, 0x0, 0x00FF_0000, 0x0, 2, 1);
alteration_property_impl!(0x0, 0x0, 0x0, 0x0000_FF00, 3, 2);
alteration_property_impl!(0x0, 0x0, 0x0, 0x0000_00FF, 3, 3);

fn alteration_from_octet(octet: u8) -> Result<Option<Alteration>> {
    use Alteration::*;
    match u8::from(octet.nibble(0)) {
        0x0 => Ok(None),
        0x1 => Ok(Some(Add(octet.nibble(1)))),
        0x2 => Ok(Some(Subtract(octet.nibble(1)))),
        0x3 => Ok(Some(Raise(octet.nibble(1)))),
        0x4 => Ok(Some(Lower(octet.nibble(1)))),
        _ => Err(Error::InvalidData("Couldn't interpret alteration field")),
    }
}

fn alteration_into_octet(alteration: Option<Alteration>) -> u8 {
    use Alteration::*;
    match alteration {
        None => 0x0,
        Some(Add(degree)) => 0x10 | u8::from(degree),
        Some(Subtract(degree)) => 0x20 | u8::from(degree),
        Some(Raise(degree)) => 0x30 | u8::from(degree),
        Some(Lower(degree)) => 0x40 | u8::from(degree),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{flex_data::tonic, traits::Grouped};
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = SetChordName::<[u32; 4]>::new();
        message.set_group(u4::new(0x7));
        message.set_optional_channel(Some(u4::new(0xB)));
        message.set_tonic_sharps_flats(SharpsFlats::Flat);
        message.set_tonic(tonic::Tonic::G);
        message.set_chord_type(ChordType::Major7th);
        message.set_chord_alteration1(Some(Alteration::Raise(u4::new(0x5))));
        message.set_chord_alteration2(Some(Alteration::Add(u4::new(0x9))));
        message.set_chord_alteration3(Some(Alteration::Lower(u4::new(0xB))));
        message.set_chord_alteration4(None);
        message.set_bass_sharps_flats(SharpsFlats::Sharp);
        message.set_bass_note(tonic::Tonic::A);
        message.set_bass_chord_type(ChordType::Minor9th);
        message.set_bass_alteration1(None);
        message.set_bass_alteration2(Some(Alteration::Subtract(u4::new(0x0))));
        assert_eq!(
            message,
            SetChordName([0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,]),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .optional_channel(),
            Some(u4::new(0xB))
        );
    }

    #[test]
    fn tonic_sharps_flats() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .tonic_sharps_flats(),
            SharpsFlats::Flat,
        );
    }

    #[test]
    fn tonic() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .tonic(),
            tonic::Tonic::G,
        );
    }

    #[test]
    fn chord_type() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .chord_type(),
            ChordType::Major7th,
        );
    }

    #[test]
    fn chord_alteration1() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .chord_alteration1(),
            Some(Alteration::Raise(u4::new(0x5))),
        );
    }

    #[test]
    fn chord_alteration2() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .chord_alteration2(),
            Some(Alteration::Add(u4::new(0x9))),
        );
    }

    #[test]
    fn chord_alteration3() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .chord_alteration3(),
            Some(Alteration::Lower(u4::new(0xB))),
        );
    }

    #[test]
    fn chord_alteration4() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .chord_alteration4(),
            None,
        );
    }

    #[test]
    fn bass_sharps_flats() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .bass_sharps_flats(),
            SharpsFlats::Sharp,
        );
    }

    #[test]
    fn bass_note() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .bass_note(),
            tonic::Tonic::A,
        );
    }

    #[test]
    fn bass_chord_type() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .bass_chord_type(),
            ChordType::Minor9th,
        );
    }

    #[test]
    fn bass_alteration1() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .bass_alteration1(),
            None,
        );
    }

    #[test]
    fn bass_alteration2() {
        assert_eq!(
            SetChordName::try_from(&[0xD70B_0006, 0xF703_3519, 0x4B00_0000, 0x110A_0020,][..])
                .unwrap()
                .bass_alteration2(),
            Some(Alteration::Subtract(u4::new(0x0))),
        );
    }
}
