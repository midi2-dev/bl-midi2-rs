use crate::Packet;

#[derive(
    Debug,
    PartialEq,
)]
pub enum Message {
    MidiTimeCode {
        time_code: u8,
    },
    SongPositionPointer {
        least_significant_bit: u8,
        most_significant_bit: u8,
    },
    SongSelect {
        song_number: u8,
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
pub enum MessageParseError {
    UnsupportedStatus(u8),
    IncorrectMessageType(u8),
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = MessageParseError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match p.nibble(0) {
            1 => match p.octet(1) {
                0xF1 => Ok(Message::MidiTimeCode {
                    time_code: p.octet(2),
                }),
                0xF2 => Ok(Message::SongPositionPointer {
                    least_significant_bit: p.octet(2),
                    most_significant_bit: p.octet(3),
                }), 
                0xF3 => Ok(Message::SongSelect {
                    song_number: p.octet(2),
                }),
                0xF6 => Ok(Message::TuneRequest),
                0xF8 => Ok(Message::TimingClock),
                0xFA => Ok(Message::Start),
                0xFB => Ok(Message::Continue),
                0xFC => Ok(Message::Stop),
                0xFE => Ok(Message::ActiveSensing),
                0xFF => Ok(Message::Reset),
                status => Err(MessageParseError::UnsupportedStatus(status)),
            },
            wrong_type => Err(MessageParseError::IncorrectMessageType(wrong_type)),
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
    byte1: Option<u8>,
    byte2: Option<u8>,
) -> Packet {
    let mut p = Packet {
        data: [
            0x1000_0000,
            0x0,
            0x0,
            0x0,
        ],
    }
    .set_octet(1, status);

    if let Some(b) = byte1 {
        p = p.set_octet(2, b);
    }

    if let Some(b) = byte2 {
        p = p.set_octet(3, b);
    }

    p
}


#[cfg(test)]
mod message_from_packet {
    use super::*;

    #[test]
    fn wrong_type() {
        assert_eq!(
            Message::try_from(Packet{data: [0x2000_0000,0x0,0x0,0x0]}),
            Err(MessageParseError::IncorrectMessageType(0x2)),
        );
    }

    #[test]
    fn midi_time_code() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10F1_3100,0x0,0x0,0x0]}),
            Ok(Message::MidiTimeCode{ time_code: 49 }),
        );
    }

    #[test]
    fn song_position_pointer() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10F2_B4D9,0x0,0x0,0x0]}),
            Ok(Message::SongPositionPointer { 
                least_significant_bit: 0xB4,
                most_significant_bit: 0xD9,
            }),
        );
    }

    #[test]
    fn song_select() {
        assert_eq!(
            Message::try_from(Packet{data: [0x10F3_4200,0x0,0x0,0x0]}),
            Ok(Message::SongSelect{ song_number: 0x42 }),
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
mod packet_from_message {
    use super::*;

    #[test]
    fn midi_time_code() {
        assert_eq!(
            Packet::from(Message::MidiTimeCode {
                time_code: 0xAA
            }),
            Packet{ data: [ 0x10F1_AA00, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn song_position_pointer() {
        assert_eq!(
            Packet::from(Message::SongPositionPointer {
                least_significant_bit: 0x31,
                most_significant_bit: 0x41,
            }),
            Packet{ data: [ 0x10F2_3141, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn song_select() {
        assert_eq!(
            Packet::from(Message::SongSelect {
                song_number: 0xEB
            }),
            Packet{ data: [ 0x10F3_EB00, 0x0, 0x0, 0x0 ] },
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