use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;

const OP_CODE: u32 = 0xF3;

#[midi2_attr::generate_message(Grouped)]
struct SongSelect {
    ump_type: Property<
        NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<
        NumericalConstant<OP_CODE>,
        UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
        BytesSchema<0xFF, 0x0, 0x0>,
    >,
    song: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            SongSelectMessage::builder()
                .group(u4::new(0xA))
                .song(u7::new(0x4F))
                .build(),
            Ok(SongSelectMessage::Owned(SongSelectOwned([
                0x1AF3_4F00,
                0x0,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            SongSelectMessage::from_data(&[0x1AF3_4F00, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn song() {
        assert_eq!(
            SongSelectMessage::from_data(&[0x1AF3_4F00, 0x0, 0x0, 0x0])
                .unwrap()
                .song(),
            u7::new(0x4F),
        );
    }

    #[test]
    fn from_byte_data() {
        assert_eq!(
            SongSelectMessage::from_byte_data(&[0xF3, 0x4F]),
            Ok(SongSelectMessage::Owned(SongSelectOwned([
                0x10F3_4F00,
                0x0,
                0x0,
                0x0
            ])))
        )
    }

    #[test]
    fn copy_byte_data() {
        assert_eq!(
            SongSelectMessage::from_data(&[0x10F3_4F00, 0x0, 0x0, 0x0])
                .unwrap()
                .write_byte_data(&mut [0x0; 3]),
            &[0xF3, 0x4F, 0x0]
        );
    }
}
