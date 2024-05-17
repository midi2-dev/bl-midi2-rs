MIDI 2.0 Flex Data Messages

## Basic Usage

```rust
use midi2::{
    prelude::*,
    flex_data::{FlexData, FlexDataMessage},
};

let message = FlexData::try_from(&[
    0xD70B_0006,
    0xF703_3519,
    0x4B00_0000,
    0x110A_0020
][..]).expect("Valid data");

// all flex_data messages are grouped
println!("Group: {}", message.group());
// all flex_data messages have a bank property
println!("Bank: {:?}", message.bank());

match message {
    FlexData::SetChordName(m) => {
        println!("Set Chord Name {:?}", m.data());
        // some flex_data messages have an optional `channel` field
        println!("Channel: {:?}", m.optional_channel());
    },
    FlexData::ComposerName(m) => {
        println!("Composer Name {:?}", m.data());
        // string properties of flex_data messages carrying string data
        // can be read as a std::string::String (std feature enabled)
        println!("Name {:?}", m.name());
        // or as an iterator over the utf-8 bytes (no_std freindly)
        println!("Name bytes {:?}", m.name_bytes().collect::<Vec<u8>>());
    }
    FlexData::SetKeySignature(m) => println!("Set Key Signature {:?}", m.data()),
    FlexData::SetMetronome(m) => println!("Set Metronome {:?}", m.data()),
    FlexData::SetTempo(m) => println!("Set Tempo {:?}", m.data()),
    FlexData::SetTimeSignature(m) => println!("Set Time Signature {:?}", m.data()),
    FlexData::UnknownMetadataText(m) => println!("Unknown Metadata Text {:?}", m.data()),
    FlexData::ProjectName(m) => println!("Project Name {:?}", m.data()),
    FlexData::CompositionName(m) => println!("Composition Name {:?}", m.data()),
    FlexData::MidiClipName(m) => println!("Midi Clip Name {:?}", m.data()),
    FlexData::CopyrightNotice(m) => println!("Copyright Notice {:?}", m.data()),
    FlexData::LyricistName(m) => println!("Lyricist Name {:?}", m.data()),
    FlexData::ArrangerName(m) => println!("Arranger Name {:?}", m.data()),
    FlexData::PublisherName(m) => println!("Publisher Name {:?}", m.data()),
    FlexData::PrimaryPerformerName(m) => println!("Primary Performer Name {:?}", m.data()),
    FlexData::AccompanyingPerformerName(m) => println!("Accompanying Performer Name {:?}", m.data()),
    FlexData::RecordingDate(m) => println!("Recording Date {:?}", m.data()),
    FlexData::RecordingLocation(m) => println!("Recording Location {:?}", m.data()),
    FlexData::UnknownPerformanceText(m) => println!("Unknown Performance Text {:?}", m.data()),
    FlexData::Lyrics(m) => println!("Lyrics {:?}", m.data()),
    FlexData::LyricsLanguage(m) => println!("Lyrics Language {:?}", m.data()),
    FlexData::Ruby(m) => println!("Ruby {:?}", m.data()),
    FlexData::RubyLanguage(m) => println!("Ruby Language {:?}", m.data()),
    _ => {},
}
```

## Dynamically Sized

Some flex_data messages are fixed size and some are dynamically sized.
All default constructed flex_data messages will fit into a `[u32; 4]`.
