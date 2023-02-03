use midi2::message::Midi2Message;

struct DataBuffer<const S: usize> {
    data: [u32; S],
    index: usize,
}

impl<const S: usize> DataBuffer<S> {
    fn new() -> DataBuffer<S> {
        DataBuffer {
            data: [0; S],
            index: 0,
        }
    }
    fn serialize<M: Midi2Message>(&mut self, m: M) {
        m.to_ump(&mut self.data[self.index..self.index + 4]);
        self.index += 4;
    }
    fn data(&self) -> &[u32] {
        &self.data[..self.index]
    }
}

mod midi1_channel_voice {
    use super::*;
    use midi2::message::midi1_channel_voice::*;

    #[test]
    fn round_trip() {
        let mut buffer = DataBuffer::<28>::new();

        buffer.serialize(
            NoteOnMessage::builder()
                .group(ux::u4::new(0x1))
                .channel(ux::u4::new(0x9))
                .note(ux::u7::new(0x3B))
                .velocity(ux::u7::new(0x29))
                .build(),
        );
        buffer.serialize(
            NoteOffMessage::builder()
                .group(ux::u4::new(0x1))
                .channel(ux::u4::new(0x9))
                .note(ux::u7::new(0x3B))
                .velocity(ux::u7::new(0x29))
                .build(),
        );
        buffer.serialize(
            ChannelPressureMessage::builder()
                .group(ux::u4::new(0x1))
                .channel(ux::u4::new(0x9))
                .pressure(ux::u7::new(0x10))
                .build(),
        );
        buffer.serialize(
            ControlChangeMessage::builder()
                .group(ux::u4::new(0x1))
                .channel(ux::u4::new(0x9))
                .controller(ux::u7::new(0x26))
                .value(ux::u7::new(0x4F))
                .build(),
        );
        buffer.serialize(
            KeyPressureMessage::builder()
                .group(ux::u4::new(0x1))
                .channel(ux::u4::new(0x9))
                .note(ux::u7::new(0x01))
                .pressure(ux::u7::new(0x02))
                .build(),
        );
        buffer.serialize(
            PitchBendMessage::builder()
                .group(ux::u4::new(0x1))
                .channel(ux::u4::new(0x9))
                .bend(ux::u14::new(0x3ACE))
                .build(),
        );
        buffer.serialize(
            ProgramChangeMessage::builder()
                .group(ux::u4::new(0x1))
                .channel(ux::u4::new(0x9))
                .program(ux::u7::new(0x0F))
                .build(),
        );

        let mut chunks = buffer.data().chunks(4);

        assert_eq!(NoteOnMessage::validate_ump(chunks.next().unwrap()), Ok(()));
        assert_eq!(NoteOffMessage::validate_ump(chunks.next().unwrap()), Ok(()));
        assert_eq!(
            ChannelPressureMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            ControlChangeMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            KeyPressureMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            PitchBendMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            ProgramChangeMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
    }
}

mod midi2_channel_voice {
    use super::*;
    use midi2::message::midi2_channel_voice::*;

    #[test]
    fn round_trip() {
        let mut buffer = DataBuffer::<64>::new();

        buffer.serialize(
            AssignableControllerMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .bank(ux::u7::new(0x4F))
                .index(ux::u7::new(0x77))
                .data(0x1234_5678)
                .build(),
        );
        buffer.serialize(
            AssignablePerNoteControllerMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .note(ux::u7::new(0x20))
                .index(0xFF)
                .data(0x1234_5678)
                .build(),
        );
        buffer.serialize(
            ChannelPressureMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .data(0x1234_5678)
                .build(),
        );
        buffer.serialize(
            ControlChangeMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .index(ux::u7::new(0x50))
                .data(0x1234_5678)
                .build(),
        );
        buffer.serialize(
            ControlChangeMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .index(ux::u7::new(0x50))
                .data(0x1234_5678)
                .build(),
        );
        buffer.serialize(
            KeyPressureMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .note(ux::u7::new(0x26))
                .data(0x1234_5678)
                .build(),
        );
        buffer.serialize(
            NoteOffMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .note(ux::u7::new(0x26))
                .velocity(0x9999)
                .attribute(NoteAttribute::ManufacturerSpecific(0x4321))
                .build(),
        );
        buffer.serialize(
            NoteOnMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .note(ux::u7::new(0x26))
                .velocity(0x9999)
                .attribute(NoteAttribute::Pitch7_9 {
                    note: ux::u7::new(0x4C),
                    pitch_up: ux::u9::new(0b1_0011_1011),
                })
                .build(),
        );
        buffer.serialize(
            PerNoteManagementMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .note(ux::u7::new(0x32))
                .detach(false)
                .reset(true)
                .build(),
        );
        buffer.serialize(
            PerNotePitchBendMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .note(ux::u7::new(0x26))
                .data(0x1234_5678)
                .build(),
        );
        buffer.serialize(
            PitchBendMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .data(0x1234_5678)
                .build(),
        );
        buffer.serialize(
            ProgramChangeMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .program(ux::u7::new(0x66))
                .build(),
        );
        buffer.serialize(
            RegisteredControllerMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .bank(ux::u7::new(0x29))
                .index(ux::u7::new(0x11))
                .data(0x8765_4321)
                .build(),
        );
        buffer.serialize(
            RegisteredPerNoteControllerMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .note(ux::u7::new(0x36))
                .controller(Controller::Modulation(0x1234_5678))
                .build(),
        );
        buffer.serialize(
            RelativeAssignableControllerMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .bank(ux::u7::new(0x29))
                .index(ux::u7::new(0x11))
                .data(0x8765_4321)
                .build(),
        );
        buffer.serialize(
            RelativeRegisteredControllerMessage::builder()
                .group(ux::u4::new(0x6))
                .channel(ux::u4::new(0xB))
                .bank(ux::u7::new(0x29))
                .index(ux::u7::new(0x11))
                .data(0x8765_4321)
                .build(),
        );

        let mut chunks = buffer.data().chunks(4);

        assert_eq!(
            AssignableControllerMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            AssignablePerNoteControllerMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            ChannelPressureMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            ControlChangeMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            ControlChangeMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            KeyPressureMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(NoteOffMessage::validate_ump(chunks.next().unwrap()), Ok(()));
        assert_eq!(NoteOnMessage::validate_ump(chunks.next().unwrap()), Ok(()));
        assert_eq!(
            PerNoteManagementMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            PerNotePitchBendMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            PitchBendMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            ProgramChangeMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            RegisteredControllerMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            RegisteredPerNoteControllerMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            RelativeAssignableControllerMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            RelativeRegisteredControllerMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
    }
}

mod system_common {
    use super::*;
    use midi2::message::system_common::*;

    #[test]
    fn round_trip() {
        let mut buffer = DataBuffer::<44>::new();

        buffer.serialize(
            ActiveSensingMessage::builder()
                .group(ux::u4::new(0xA))
                .build(),
        );
        buffer.serialize(ContinueMessage::builder().group(ux::u4::new(0xA)).build());
        buffer.serialize(ResetMessage::builder().group(ux::u4::new(0xA)).build());
        buffer.serialize(
            SongPositionPointerMessage::builder()
                .group(ux::u4::new(0xA))
                .position(ux::u14::new(0x3FFF))
                .build(),
        );
        buffer.serialize(
            SongSelectMessage::builder()
                .group(ux::u4::new(0xA))
                .song(ux::u7::new(0x2A))
                .build(),
        );
        buffer.serialize(StartMessage::builder().group(ux::u4::new(0xA)).build());
        buffer.serialize(StartMessage::builder().group(ux::u4::new(0xA)).build());
        buffer.serialize(StopMessage::builder().group(ux::u4::new(0xA)).build());
        buffer.serialize(
            TimeCodeMessage::builder()
                .group(ux::u4::new(0xA))
                .time_code(ux::u7::new(0x11))
                .build(),
        );
        buffer.serialize(
            TimingClockMessage::builder()
                .group(ux::u4::new(0xA))
                .build(),
        );
        buffer.serialize(
            TuneRequestMessage::builder()
                .group(ux::u4::new(0xA))
                .build(),
        );

        let mut chunks = buffer.data().chunks(4);

        assert_eq!(
            ActiveSensingMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            ContinueMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(ResetMessage::validate_ump(chunks.next().unwrap()), Ok(()));
        assert_eq!(
            SongPositionPointerMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            SongSelectMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(StartMessage::validate_ump(chunks.next().unwrap()), Ok(()));
        assert_eq!(StartMessage::validate_ump(chunks.next().unwrap()), Ok(()));
        assert_eq!(StopMessage::validate_ump(chunks.next().unwrap()), Ok(()));
        assert_eq!(
            TimeCodeMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            TimingClockMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
        assert_eq!(
            TuneRequestMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
    }
}

mod utility {
    use super::*;
    use midi2::message::utility::*;

    #[test]
    fn round_trip() {
        let mut buffer = DataBuffer::<44>::new();

        buffer.serialize(
            TimeStampMessage::builder()
                .group(ux::u4::new(0x8))
                .time_stamp(ux::u20::new(0xF_FFFF))
                .build(),
        );

        let mut chunks = buffer.data().chunks(4);

        assert_eq!(
            TimeStampMessage::validate_ump(chunks.next().unwrap()),
            Ok(())
        );
    }
}

mod system_exclusive_7bit {
    use super::*;
    use midi2::message::system_exclusive_7bit::*;

    #[test]
    fn round_trip() {
        let mut buffer = DataBuffer::<16>::new();

        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x8))
                .status(Status::Complete)
                .data(&[
                    ux::u7::new(0x59),
                    ux::u7::new(0x5A),
                    ux::u7::new(0x5B),
                    ux::u7::new(0x5C),
                ])
                .build(),
        );
        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x8))
                .status(Status::Begin)
                .data(&[
                    ux::u7::new(0x59),
                    ux::u7::new(0x5A),
                    ux::u7::new(0x5B),
                    ux::u7::new(0x5C),
                    ux::u7::new(0x5D),
                    ux::u7::new(0x5E),
                ])
                .build(),
        );
        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x8))
                .status(Status::Continue)
                .data(&[ux::u7::new(0x5F)])
                .build(),
        );
        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x8))
                .status(Status::End)
                .build(),
        );

        let mut chunks = buffer.data().chunks(4);

        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::Complete
        );
        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::Begin
        );
        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::Continue
        );
        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::End
        );
    }
}

mod system_exclusive_8bit {
    use super::*;
    use midi2::message::system_exclusive_8bit::*;

    #[test]
    fn round_trip() {
        let mut buffer = DataBuffer::<20>::new();

        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x2))
                .status(Status::Complete)
                .stream_id(0xB1)
                .data(&[
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                ])
                .build(),
        );
        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x2))
                .stream_id(0xB1)
                .status(Status::Begin)
                .data(&[0x10, 0x11, 0x12, 0x13, 0x14])
                .build(),
        );
        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x2))
                .stream_id(0xB1)
                .status(Status::Continue)
                .data(&[0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C])
                .build(),
        );
        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x2))
                .stream_id(0xB1)
                .status(Status::UnexpectedEnd(Validity::Valid))
                .build(),
        );
        buffer.serialize(
            Message::builder()
                .group(ux::u4::new(0x2))
                .stream_id(0xB1)
                .status(Status::UnexpectedEnd(Validity::Invalid))
                .build(),
        );

        let mut chunks = buffer.data().chunks(4);

        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::Complete
        );
        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::Begin
        );
        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::Continue
        );
        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::UnexpectedEnd(Validity::Valid)
        );
        assert_eq!(
            Message::from_ump(chunks.next().unwrap()).status(),
            Status::UnexpectedEnd(Validity::Invalid)
        );
    }
}
