use crate::{
    util::Truncate,
    error::Error,
};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
#[non_exhaustive]
pub enum Controller {
    Modulation(u32),
    Breath(u32),
    Pitch7_25 {
        note: ux::u7,
        pitch_up: ux::u25,
    },
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
    SoundController {
        index: u8,
        data: u32,
    },
    EffectDepth {
        index: u8,
        data: u32,
    }
}

pub fn try_from_index_and_data(index: u8, data: u32) -> Result<Controller, Error> {
    match index {
        1 => Ok(Controller::Modulation(data)),
        2 => Ok(Controller::Breath(data)),
        3 => Ok(Controller::Pitch7_25 { 
            note: ux::u7::try_from(data >> 25).unwrap(),
            pitch_up: data.truncate(),
        }),
        7 => Ok(Controller::Volume(data)),
        8 => Ok(Controller::Balance(data)),
        10 => Ok(Controller::Pan(data)),
        11 => Ok(Controller::Expression(data)),
        70 => Ok(Controller::SoundController { index: 1, data }),
        71 => Ok(Controller::SoundController { index: 2, data }),
        72 => Ok(Controller::SoundController { index: 3, data }),
        73 => Ok(Controller::SoundController { index: 4, data }),
        74 => Ok(Controller::SoundController { index: 5, data }),
        75 => Ok(Controller::SoundController { index: 6, data }),
        76 => Ok(Controller::SoundController { index: 7, data }),
        77 => Ok(Controller::SoundController { index: 8, data }),
        78 => Ok(Controller::SoundController { index: 9, data }),
        79 => Ok(Controller::SoundController { index: 10, data }),
        91 => Ok(Controller::EffectDepth { index: 1, data }),
        92 => Ok(Controller::EffectDepth { index: 2, data }),
        93 => Ok(Controller::EffectDepth { index: 3, data }),
        94 => Ok(Controller::EffectDepth { index: 4, data }),
        95 => Ok(Controller::EffectDepth { index: 5, data }),
        _ => Err(Error::InvalidData)
    }
}

pub fn to_index_and_data(c: Controller) -> (u8, u32) {
    match c {
        Controller::Modulation(data) => (1, data),
        Controller::Breath(data) => (2, data),
        Controller::Pitch7_25{ note, pitch_up } => (3, u32::from(note << 25) | u32::from(pitch_up)),
        Controller::Volume(data) => (7, data),
        Controller::Balance(data) => (8, data),
        Controller::Pan(data) => (10, data),
        Controller::Expression(data) => (11, data),
        Controller::SoundController{ index: 1, data } => (70, data),
        Controller::SoundVariation(data) => (70, data),
        Controller::SoundController{ index: 2, data } => (71, data),
        Controller::Timbre(data) => (71, data),
        Controller::SoundController{ index: 3, data } => (72, data),
        Controller::ReleaseTime(data) => (72, data),
        Controller::SoundController{ index: 4, data } => (73, data),
        Controller::AttackTime(data) => (73, data),
        Controller::SoundController{ index: 5, data } => (74, data),
        Controller::Brightness(data) => (74, data),
        Controller::SoundController{ index: 6, data } => (75, data),
        Controller::DecayTime(data) => (75, data),
        Controller::SoundController{ index: 7, data } => (76, data),
        Controller::VebratoRate(data) => (76, data),
        Controller::SoundController{ index: 8, data } => (77, data),
        Controller::VebratoDepth(data) => (77, data),
        Controller::SoundController{ index: 9, data } => (78, data),
        Controller::VebratoDelay(data) => (78, data),
        Controller::SoundController{ index: 10, data } => (79, data),
        Controller::Undefined(data) => (79, data),
        Controller::EffectDepth{ index: 1, data } => (91, data),
        Controller::ReverbSendLevel(data) => (91, data),
        Controller::EffectDepth{ index: 2, data } => (92, data),
        Controller::EffectDepth{ index: 3, data } => (93, data),
        Controller::ChorusSendLevel(data) => (91, data),
        Controller::EffectDepth{ index: 4, data } => (94, data),
        _ => unreachable!(),
    }
}