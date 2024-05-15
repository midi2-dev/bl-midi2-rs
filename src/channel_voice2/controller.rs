use crate::{
    detail::{property, BitOps, Truncate},
    error::InvalidData,
};
use ux::{u25, u7};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Controller {
    Modulation(u32),
    Breath(u32),
    Pitch7_25 { note: u7, pitch_up: u25 },
    Volume(u32),
    Balance(u32),
    Pan(u32),
    Expression(u32),
    SoundVariation(u32),
    Timbre(u32),
    ReleaseTime(u32),
    AttackTime(u32),
    Brightness(u32),
    DecayTime(u32),
    VebratoRate(u32),
    VebratoDepth(u32),
    VebratoDelay(u32),
    Undefined(u32),
    ReverbSendLevel(u32),
    ChorusSendLevel(u32),
    SoundController { index: u8, data: u32 },
    EffectDepth { index: u8, data: u32 },
}

pub fn validate_index(index: u8) -> Result<(), crate::error::InvalidData> {
    match index {
        1 => Ok(()),
        2 => Ok(()),
        3 => Ok(()),
        7 => Ok(()),
        8 => Ok(()),
        10 => Ok(()),
        11 => Ok(()),
        70 => Ok(()),
        71 => Ok(()),
        72 => Ok(()),
        73 => Ok(()),
        74 => Ok(()),
        75 => Ok(()),
        76 => Ok(()),
        77 => Ok(()),
        78 => Ok(()),
        79 => Ok(()),
        91 => Ok(()),
        92 => Ok(()),
        93 => Ok(()),
        94 => Ok(()),
        95 => Ok(()),
        _ => Err(InvalidData("Couldn't interpret controller index")),
    }
}

pub fn from_index_and_data(index: u8, data: u32) -> Controller {
    match index {
        1 => Controller::Modulation(data),
        2 => Controller::Breath(data),
        3 => Controller::Pitch7_25 {
            note: u7::try_from(data >> 25).unwrap(),
            pitch_up: data.truncate(),
        },
        7 => Controller::Volume(data),
        8 => Controller::Balance(data),
        10 => Controller::Pan(data),
        11 => Controller::Expression(data),
        70 => Controller::SoundController { index: 1, data },
        71 => Controller::SoundController { index: 2, data },
        72 => Controller::SoundController { index: 3, data },
        73 => Controller::SoundController { index: 4, data },
        74 => Controller::SoundController { index: 5, data },
        75 => Controller::SoundController { index: 6, data },
        76 => Controller::SoundController { index: 7, data },
        77 => Controller::SoundController { index: 8, data },
        78 => Controller::SoundController { index: 9, data },
        79 => Controller::SoundController { index: 10, data },
        91 => Controller::EffectDepth { index: 1, data },
        92 => Controller::EffectDepth { index: 2, data },
        93 => Controller::EffectDepth { index: 3, data },
        94 => Controller::EffectDepth { index: 4, data },
        95 => Controller::EffectDepth { index: 5, data },
        _ => panic!("Invalid index"),
    }
}

pub fn to_index_and_data(c: Controller) -> (u8, u32) {
    match c {
        Controller::Modulation(data) => (1, data),
        Controller::Breath(data) => (2, data),
        Controller::Pitch7_25 { note, pitch_up } => {
            (3, u32::from(note << 25) | u32::from(pitch_up))
        }
        Controller::Volume(data) => (7, data),
        Controller::Balance(data) => (8, data),
        Controller::Pan(data) => (10, data),
        Controller::Expression(data) => (11, data),
        Controller::SoundController { index: 1, data } => (70, data),
        Controller::SoundVariation(data) => (70, data),
        Controller::SoundController { index: 2, data } => (71, data),
        Controller::Timbre(data) => (71, data),
        Controller::SoundController { index: 3, data } => (72, data),
        Controller::ReleaseTime(data) => (72, data),
        Controller::SoundController { index: 4, data } => (73, data),
        Controller::AttackTime(data) => (73, data),
        Controller::SoundController { index: 5, data } => (74, data),
        Controller::Brightness(data) => (74, data),
        Controller::SoundController { index: 6, data } => (75, data),
        Controller::DecayTime(data) => (75, data),
        Controller::SoundController { index: 7, data } => (76, data),
        Controller::VebratoRate(data) => (76, data),
        Controller::SoundController { index: 8, data } => (77, data),
        Controller::VebratoDepth(data) => (77, data),
        Controller::SoundController { index: 9, data } => (78, data),
        Controller::VebratoDelay(data) => (78, data),
        Controller::SoundController { index: 10, data } => (79, data),
        Controller::Undefined(data) => (79, data),
        Controller::EffectDepth { index: 1, data } => (91, data),
        Controller::ReverbSendLevel(data) => (91, data),
        Controller::EffectDepth { index: 2, data } => (92, data),
        Controller::EffectDepth { index: 3, data } => (93, data),
        Controller::ChorusSendLevel(data) => (91, data),
        Controller::EffectDepth { index: 4, data } => (94, data),
        _ => unreachable!(),
    }
}

pub struct ControllerProperty;

impl<B: crate::buffer::Ump> property::Property<B> for ControllerProperty {
    type Type = Controller;
}

impl<'a, B: crate::buffer::Ump> property::ReadProperty<'a, B> for ControllerProperty {
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        let buffer = buffer.buffer();
        validate_index(buffer[0].octet(3))
    }
    fn read(buffer: &'a B) -> Self::Type {
        let buffer = buffer.buffer();
        from_index_and_data(buffer[0].octet(3), buffer[1])
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> property::WriteProperty<B>
    for ControllerProperty
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        let buffer = buffer.buffer_mut();
        let (index, controller_data) = to_index_and_data(v);
        buffer[0].set_octet(3, index);
        buffer[1] = controller_data;
    }
}

impl core::default::Default for Controller {
    /// Default value is Controller::Modulation(0x0)
    fn default() -> Self {
        Controller::Modulation(0x0)
    }
}
