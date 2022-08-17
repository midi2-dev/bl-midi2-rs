use crate::{
    Packet,
    helpers::mask,
};

#[derive(
    Debug,
    PartialEq,
)]
pub enum Message {
    MidiTimeCode {
        time_code: ux::u7,
    },
    SongPositionPointer {
        least_significant_bit: ux::u7,
        most_significant_bit: ux::u7,
    },
    SongSelect {
        song_number: ux::u7,
    },
    TuneRequest,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
}

#[derive(
    Debug,
    PartialEq,
)]
pub enum DeserializeError {
    UnsupportedStatus(u8),
    IncorrectMessageType(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = DeserializeError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match u8::from(p.nibble(0)) {
            1 => match u8::from(p.octet(1)) {
                0xF1 => Ok(Message::MidiTimeCode {
                    time_code: mask(p.octet(2)),
                }),
                0xF2 => Ok(Message::SongPositionPointer {
                    least_significant_bit: mask(p.octet(2)),
                    most_significant_bit: mask(p.octet(3)),
                }), 
                0xF3 => Ok(Message::SongSelect {
                    song_number: mask(p.octet(2)),
                }),
                0xF6 => Ok(Message::TuneRequest),
                0xF8 => Ok(Message::TimingClock),
                0xFA => Ok(Message::Start),
                0xFB => Ok(Message::Continue),
                0xFC => Ok(Message::Stop),
                0xFE => Ok(Message::ActiveSensing),
                0xFF => Ok(Message::Reset),
                status => Err(DeserializeError::UnsupportedStatus(status)),
            },
            wrong_type => Err(DeserializeError::IncorrectMessageType(wrong_type)),
        }
    }
}

impl std::convert::From<Message> for Packet {
    fn from(m: Message) -> Self {
        match m {
            Message::MidiTimeCode { 
                time_code
            } => message_packet(
                0xF1, 
                Some(time_code), 
                None
            ),
            Message::SongPositionPointer { 
                least_significant_bit,
                most_significant_bit,
            } => message_packet(
                0xF2, 
                Some(least_significant_bit), 
                Some(most_significant_bit),
            ),
            Message::SongSelect { 
                song_number,
            } => message_packet(
                0xF3, 
                Some(song_number), 
                None,
            ),
            Message::TuneRequest => message_packet(0xF6, None, None),
            Message::TimingClock => message_packet(0xF8, None, None),
            Message::Start => message_packet(0xFA, None, None),
            Message::Continue => message_packet(0xFB, None, None),
            Message::Stop => message_packet(0xFC, None, None),
            Message::ActiveSensing => message_packet(0xFE, None, None),
            Message::Reset => message_packet(0xFF, None, None),
        }
    }
}

fn message_packet(
    status: u8,
    byte1: Option<ux::u7>,
    byte2: Option<ux::u7>,
) -> Packet {
    let mut p = Packet {
        data: [
            0x1000_0000,
            0x0,
            0x0,
            0x0,
        ],
    }
    .set_octet(1, mask(status));

    if let Some(b) = byte1 {
        p = p.set_octet(2, b.into());
    }

    if let Some(b) = byte2 {
        p = p.set_octet(3, b.into());
    }

    p
}


#[cfg(test)]
mod deserialize {
    use super::*;

    #[test]
    fn wrong_type() {
        assert_eq!(
            Message::try_from(Packet{data: [0x2000_0000,0x0,0x0,0x0]}),
            Err(DeserializeError::IncorrectMessageType(0x2)),
        );
    }

    #[test]
    fn midi_time_code() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10F1_3100,0x0,0x0,0x0]}),
            Ok(Message::MidiTimeCode{ time_code: ux::u7::new(49) }),
        );
    }

    #[test]
    fn song_position_pointer() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10F2_2449,0x0,0x0,0x0]}),
            Ok(Message::SongPositionPointer { 
                least_significant_bit: ux::u7::new(0x24),
                most_significant_bit: ux::u7::new(0x49),
            }),
        );
    }

    #[test]
    fn song_select() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10F3_4200,0x0,0x0,0x0]}),
            Ok(Message::SongSelect{ song_number: ux::u7::new(0x42) }),
        );
    }

    #[test]
    fn tune_request() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10F6_0000,0x0,0x0,0x0]}),
            Ok(Message::TuneRequest),
        );
    }

    #[test]
    fn timing_clock() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10F8_0000,0x0,0x0,0x0]}),
            Ok(Message::TimingClock),
        );
    }

    #[test]
    fn start() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10FA_0000,0x0,0x0,0x0]}),
            Ok(Message::Start),
        );
    }

    #[test]
    fn continue_message() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10FB_0000,0x0,0x0,0x0]}),
            Ok(Message::Continue),
        );
    }

    #[test]
    fn stop() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10FC_0000,0x0,0x0,0x0]}),
            Ok(Message::Stop),
        );
    }

    #[test]
    fn active_sensing() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10FE_0000,0x0,0x0,0x0]}),
            Ok(Message::ActiveSensing),
        );
    }

    #[test]
    fn reset() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10FF_0000,0x0,0x0,0x0]}),
            Ok(Message::Reset),
        );
    }
}

#[cfg(test)]
mod serialize {
    use super::*;

    #[test]
    fn midi_time_code() {
        assert_eq!(
            Packet::from(Message::MidiTimeCode {
                time_code: ux::u7::new(0x1A)
            }),
            Packet{ data: [ 0x10F1_1A00, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn song_position_pointer() {
        assert_eq!(
            Packet::from(Message::SongPositionPointer {
                least_significant_bit: ux::u7::new(0x31),
                most_significant_bit: ux::u7::new(0x41),
            }),
            Packet{ data: [ 0x10F2_3141, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn song_select() {
        assert_eq!(
            Packet::from(Message::SongSelect {
                song_number: ux::u7::new(0x5B)
            }),
            Packet{ data: [ 0x10F3_5B00, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn tune_request() {
        assert_eq!(
            Packet::from(Message::TuneRequest),
            Packet{ data: [ 0x10F6_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn timing_clock() {
        assert_eq!(
            Packet::from(Message::TimingClock),
            Packet{ data: [ 0x10F8_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn start() {
        assert_eq!(
            Packet::from(Message::Start),
            Packet{ data: [ 0x10FA_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn continue_message() {
        assert_eq!(
            Packet::from(Message::Continue),
            Packet{ data: [ 0x10FB_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn stop() {
        assert_eq!(
            Packet::from(Message::Stop),
            Packet{ data: [ 0x10FC_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn active_sensing() {
        assert_eq!(
            Packet::from(Message::ActiveSensing),
            Packet{ data: [ 0x10FE_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn reset() {
        assert_eq!(
            Packet::from(Message::Reset),
            Packet{ data: [ 0x10FF_0000, 0x0, 0x0, 0x0 ] },
        );
    }
}
