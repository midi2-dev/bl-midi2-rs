use crate::{
    Channel,
    Controller,
    Note,
    Value,
    Velocity,
    Packet,
    Program,
};

#[derive(
    Debug,
    PartialEq,
)]
pub enum ChannelVoiceMessage {
    NoteOff {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
    NoteOn {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
    KeyPressure {
        channel: Channel,
        note: Note,
        value: Value,
    },
    ControlChange {
        channel: Channel,
        controller: Controller,
        value: Value,
    },
    ProgramChange {
        channel: Channel,
        program: Program,
    },
    ChannelPressure {
        channel: Channel,
        value: Value,    
    },
    PitchBend {
        channel: Channel,
        least_significant_bit: Value,
        most_significant_bit: Value,
    },
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum ChannelVoiceMessageParseError {
    UnsupportedStatus(u8),
}

impl std::convert::TryFrom<Packet> for ChannelVoiceMessage {
    type Error = ChannelVoiceMessageParseError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        let channel = p.nibble(3);
        match p.nibble(2) {
            0x8 => Ok(ChannelVoiceMessage::NoteOff {
                channel,
                note: p.octet(2),
                velocity: p.octet(3),
            }),
            0x9 => Ok(ChannelVoiceMessage::NoteOn {
                channel,
                note: p.octet(2),
                velocity: p.octet(3),
            }),
            0xA => Ok(ChannelVoiceMessage::KeyPressure {
                channel,
                note: p.octet(2),
                value: p.octet(3),
            }),
            0xB => Ok(ChannelVoiceMessage::ControlChange {
                channel,
                controller: p.octet(2),
                value: p.octet(3),
            }),
            0xC => Ok(ChannelVoiceMessage::ProgramChange {
                channel,
                program: p.octet(2),
            }),
            0xD => Ok(ChannelVoiceMessage::ChannelPressure {
                channel,
                value: p.octet(2),
            }),
            0xE => Ok(ChannelVoiceMessage::PitchBend {
                channel,
                least_significant_bit: p.octet(2),
                most_significant_bit: p.octet(3),
            }),
            status => Err(ChannelVoiceMessageParseError::UnsupportedStatus(status)),
        }
    }
}
