use crate::error::Error;

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum Controller {
    Modulation,
    Breath,
    Pitch7_25,
    Volume,
    Balance,
    Pan,
    Expression,
    SoundVariation,
    Timbre,
    ReleaseTime,
    AttackTime,
    Brightness,
    DecayTime,
    VebratoRate,
    VebratoDepth,
    VebratoDelay,
    Undefined,
    ReverbSendLevel,
    ChorusSendLevel,
    SoundController(u8),
    EffectDepth(u8),
}

impl core::convert::TryFrom<u8> for Controller {
    type Error = Error;
    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            1 => Ok(Controller::Modulation),
            2 => Ok(Controller::Breath),
            3 => Ok(Controller::Pitch7_25),
            7 => Ok(Controller::Volume),
            8 => Ok(Controller::Balance),
            10 => Ok(Controller::Pan),
            11 => Ok(Controller::Expression),
            70 => Ok(Controller::SoundController(1)),
            71 => Ok(Controller::SoundController(2)),
            72 => Ok(Controller::SoundController(3)),
            73 => Ok(Controller::SoundController(4)),
            74 => Ok(Controller::SoundController(5)),
            75 => Ok(Controller::SoundController(6)),
            76 => Ok(Controller::SoundController(7)),
            77 => Ok(Controller::SoundController(8)),
            78 => Ok(Controller::SoundController(9)),
            79 => Ok(Controller::SoundController(10)),
            91 => Ok(Controller::EffectDepth(1)),
            92 => Ok(Controller::EffectDepth(2)),
            93 => Ok(Controller::EffectDepth(3)),
            94 => Ok(Controller::EffectDepth(4)),
            95 => Ok(Controller::EffectDepth(5)),
            _ => Err(Error::InvalidData),
        }
    }
}

impl core::convert::From<Controller> for u8 {
    fn from(c: Controller) -> Self {
        match c {
            Controller::Modulation => 1,
            2 => Ok(Controller::Breath),
            3 => Ok(Controller::Pitch7_25),
            7 => Ok(Controller::Volume),
            8 => Ok(Controller::Balance),
            10 => Ok(Controller::Pan),
            11 => Ok(Controller::Expression),
            70 => Ok(Controller::SoundController(1)),
            71 => Ok(Controller::SoundController(2)),
            72 => Ok(Controller::SoundController(3)),
            73 => Ok(Controller::SoundController(4)),
            74 => Ok(Controller::SoundController(5)),
            75 => Ok(Controller::SoundController(6)),
            76 => Ok(Controller::SoundController(7)),
            77 => Ok(Controller::SoundController(8)),
            78 => Ok(Controller::SoundController(9)),
            79 => Ok(Controller::SoundController(10)),
            91 => Ok(Controller::EffectDepth(1)),
            92 => Ok(Controller::EffectDepth(2)),
            93 => Ok(Controller::EffectDepth(3)),
            94 => Ok(Controller::EffectDepth(4)),
            95 => Ok(Controller::EffectDepth(5)),
            _ => Err(Error::InvalidData),
        }
    }
}