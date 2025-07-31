use midi2::prelude::*;

fn handle_message(buffer: &[u32]) {
    match UmpMessage::try_from(buffer) {
        Ok(UmpMessage::ChannelVoice2(m)) => {
            println!("Channel Voice2: channel: {}", m.channel());
            match m {
                channel_voice2::ChannelVoice2::NoteOn(m) => {
                    println!(
                        "Note On! note: {}, velocity: {}",
                        m.note_number(),
                        m.velocity()
                    );
                }
                channel_voice2::ChannelVoice2::NoteOff(m) => {
                    println!(
                        "Note Off! note: {}, velocity: {}",
                        m.note_number(),
                        m.velocity()
                    );
                }
                _ => {}
            }
        }
        Ok(UmpMessage::Sysex7(m)) => {
            println!(
                "Sysex 7bit: payload: {:?}",
                m.payload().collect::<Vec<u7>>()
            );
        }
        Err(e) => {
            println!("Error parsing ump buffer: {e:?}");
        }
        _ => {}
    }
}

fn main() {
    handle_message(&[0x4898_5E03, 0x6A14_E98A]); // note on
    handle_message(&[0x4288_4E01, 0x9DE6_CC6E]); // note off
    handle_message(&[0xF000_0101, 0x0000_001F]); // err - ump-stream feature not enabled
    handle_message(&[
        0x3016_0001,
        0x0203_0405,
        0x3026_0607,
        0x0809_0A0B,
        0x3026_0C0D,
        0x0E0F_1011,
        0x3026_1213,
        0x1415_1617,
        0x3036_1819,
        0x1A1B_1C1D,
    ]); // sysex7
}
