use crate::Packet;

#[derive(
    Debug,
    PartialEq,
)]
enum SystemMessage {
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
enum SystemMessageParseError {
    UnsupportedStatus(u8),
}

impl std::convert::TryFrom<Packet> for SystemMessage {
    type Error = SystemMessageParseError;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        assert_eq!(p.nibble(0), 0x1);
        match p.octet(1) {
            0xF1 => Ok(SystemMessage::MidiTimeCode {
                time_code: p.octet(2),
            }),
            0xF2 => Ok(SystemMessage::SongPositionPointer {
                least_significant_bit: p.octet(2),
                most_significant_bit: p.octet(3),
            }), 
            0xF3 => Ok(SystemMessage::SongSelect {
                song_number: p.octet(2),
            }),
            0xF6 => Ok(SystemMessage::TuneRequest),
            0xF8 => Ok(SystemMessage::TimingClock),
            0xFA => Ok(SystemMessage::Start),
            0xFB => Ok(SystemMessage::Continue),
            0xFC => Ok(SystemMessage::Stop),
            0xFE => Ok(SystemMessage::ActiveSensing),
            0xFF => Ok(SystemMessage::Reset),
            status => Err(SystemMessageParseError::UnsupportedStatus(status)),
        }
    }
}

impl std::convert::From<SystemMessage> for Packet {
    fn from(m: SystemMessage) -> Self {
        match m {
            SystemMessage::MidiTimeCode { 
                time_code
            } => system_message_packet(
                0xF1, 
                Some(time_code), 
                None
            ),
            SystemMessage::SongPositionPointer { 
                least_significant_bit,
                most_significant_bit,
            } => system_message_packet(
                0xF2, 
                Some(least_significant_bit), 
                Some(most_significant_bit),
            ),
            SystemMessage::SongSelect { 
                song_number,
            } => system_message_packet(
                0xF3, 
                Some(song_number), 
                None,
            ),
            SystemMessage::TuneRequest => 
                system_message_packet(0xF6, None, None),
            SystemMessage::TimingClock => 
                system_message_packet(0xF8, None, None),
            SystemMessage::Start => 
                system_message_packet(0xFA, None, None),
            SystemMessage::Continue => 
                system_message_packet(0xFB, None, None),
            SystemMessage::Stop => 
                system_message_packet(0xFC, None, None),
            SystemMessage::ActiveSensing => 
                system_message_packet(0xFE, None, None),
            SystemMessage::Reset => 
                system_message_packet(0xFF, None, None),
        }
    }
}

fn system_message_packet(
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
    fn midi_time_code() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10F1_3100,0x0,0x0,0x0]}),
            Ok(SystemMessage::MidiTimeCode{ time_code: 49 }),
        );
    }

    #[test]
    fn song_position_pointer() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10F2_B4D9,0x0,0x0,0x0]}),
            Ok(SystemMessage::SongPositionPointer { 
                least_significant_bit: 0xB4,
                most_significant_bit: 0xD9,
            }),
        );
    }

    #[test]
    fn song_select() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10F3_4200,0x0,0x0,0x0]}),
            Ok(SystemMessage::SongSelect{ song_number: 0x42 }),
        );
    }

    #[test]
    fn tune_request() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10F6_0000,0x0,0x0,0x0]}),
            Ok(SystemMessage::TuneRequest),
        );
    }

    #[test]
    fn timing_clock() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10F8_0000,0x0,0x0,0x0]}),
            Ok(SystemMessage::TimingClock),
        );
    }

    #[test]
    fn start() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10FA_0000,0x0,0x0,0x0]}),
            Ok(SystemMessage::Start),
        );
    }

    #[test]
    fn continue_message() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10FB_0000,0x0,0x0,0x0]}),
            Ok(SystemMessage::Continue),
        );
    }

    #[test]
    fn stop() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10FC_0000,0x0,0x0,0x0]}),
            Ok(SystemMessage::Stop),
        );
    }

    #[test]
    fn active_sensing() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10FE_0000,0x0,0x0,0x0]}),
            Ok(SystemMessage::ActiveSensing),
        );
    }

    #[test]
    fn reset() {
        assert_eq!(
            SystemMessage::try_from(Packet{data: [0x10FF_0000,0x0,0x0,0x0]}),
            Ok(SystemMessage::Reset),
        );
    }
}

#[cfg(test)]
mod packet_from_message {
    use super::*;

    #[test]
    fn midi_time_code() {
        assert_eq!(
            Packet::from(SystemMessage::MidiTimeCode {
                time_code: 0xAA
            }),
            Packet{ data: [ 0x10F1_AA00, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn song_position_pointer() {
        assert_eq!(
            Packet::from(SystemMessage::SongPositionPointer {
                least_significant_bit: 0x31,
                most_significant_bit: 0x41,
            }),
            Packet{ data: [ 0x10F2_3141, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn song_select() {
        assert_eq!(
            Packet::from(SystemMessage::SongSelect {
                song_number: 0xEB
            }),
            Packet{ data: [ 0x10F3_EB00, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn tune_request() {
        assert_eq!(
            Packet::from(SystemMessage::TuneRequest),
            Packet{ data: [ 0x10F6_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn timing_clock() {
        assert_eq!(
            Packet::from(SystemMessage::TimingClock),
            Packet{ data: [ 0x10F8_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn start() {
        assert_eq!(
            Packet::from(SystemMessage::Start),
            Packet{ data: [ 0x10FA_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn continue_message() {
        assert_eq!(
            Packet::from(SystemMessage::Continue),
            Packet{ data: [ 0x10FB_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn stop() {
        assert_eq!(
            Packet::from(SystemMessage::Stop),
            Packet{ data: [ 0x10FC_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn active_sensing() {
        assert_eq!(
            Packet::from(SystemMessage::ActiveSensing),
            Packet{ data: [ 0x10FE_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn reset() {
        assert_eq!(
            Packet::from(SystemMessage::Reset),
            Packet{ data: [ 0x10FF_0000, 0x0, 0x0, 0x0 ] },
        );
    }
}
