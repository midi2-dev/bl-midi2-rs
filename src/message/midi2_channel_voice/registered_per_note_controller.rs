use crate::message::midi2_channel_voice::controller::Controller;

const OP_CODE: u32 = 0b0000;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct RegisteredPerNoteController {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    note: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    controller: Property<Controller, UmpSchema<0x0000_00FF, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;

    #[test]
    fn builder() {
        assert_eq!(
            RegisteredPerNoteControllerOwnedPrivate::builder()
                .group(u4::new(0x4))
                .channel(u4::new(0x5))
                .note(u7::new(0x6C))
                .controller(Controller::Volume(0xE1E35E92))
                .build(),
            Ok(RegisteredPerNoteControllerOwnedPrivate(arr![
                0x4405_6C07,
                0xE1E35E92,
                0x0,
                0x0,
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            RegisteredPerNoteControllerBorrowedPrivate::<Ump>::from_data(&[
                0x4405_6C07,
                0xE1E35E92,
                0x0,
                0x0
            ])
            .unwrap()
            .group(),
            u4::new(0x4),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            RegisteredPerNoteControllerBorrowedPrivate::<Ump>::from_data(&[
                0x4405_6C07,
                0xE1E35E92,
                0x0,
                0x0
            ])
            .unwrap()
            .channel(),
            u4::new(0x5),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            RegisteredPerNoteControllerBorrowedPrivate::<Ump>::from_data(&[
                0x4405_6C07,
                0xE1E35E92,
                0x0,
                0x0
            ])
            .unwrap()
            .note(),
            u7::new(0x6C),
        );
    }

    #[test]
    fn controller() {
        assert_eq!(
            RegisteredPerNoteControllerBorrowedPrivate::<Ump>::from_data(&[
                0x4405_6C07,
                0xE1E35E92,
                0x0,
                0x0
            ])
            .unwrap()
            .controller(),
            Controller::Volume(0xE1E35E92),
        );
    }
}
