use builder_derive::Builder;
use crate::{error::Error, slice_data::SliceData};

#[derive(PartialEq, Debug, Builder)]
pub struct TestMessage {
    group: ux::u4,
    #[builder(value_default)]
    data: SliceData::<ux::u7, 4>,
    optional_field: Option<bool>,
    #[builder(default = "false")]
    default_field: bool,
}

#[test]
fn builder() {
    let _ = TestMessage::builder();
}

#[test]
fn build() {
    assert_eq!(
        TestMessage::builder()
            .group(ux::u4::new(0xA))
            .data(&[ux::u7::new(1), ux::u7::new(2)]).unwrap()
            .optional_field(true)
            .default_field(true)
            .build(),
        Ok(TestMessage {
            group: ux::u4::new(0xA),
            data: SliceData(
                [
                    ux::u7::new(1),
                    ux::u7::new(2),
                    ux::u7::new(0),
                    ux::u7::new(0),
                ],
                2
            ),
            optional_field: Some(true),
            default_field: true,
        }),
    );
}

#[test]
fn missing_fields() {
    assert_eq!(
        TestMessage::builder().build(),
        Err(Error::MissingFields),
    );
}

#[test]
fn optional_fields() {
    assert_eq!(
        TestMessage::builder()
            .group(ux::u4::new(0x2))
            .data(&[]).unwrap()
            .default_field(true)
            .build(),
        Ok(TestMessage {
            group: ux::u4::new(0x2),
            data: SliceData(
                [
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                ],
                0,
            ),
            optional_field: None,
            default_field: true,
        }),
    );
}

#[test]
fn default_fields() {
    assert_eq!(
        TestMessage::builder()
            .group(ux::u4::new(0x2))
            .data(&[]).unwrap()
            .build(),
        Ok(TestMessage {
            group: ux::u4::new(0x2),
            data: SliceData(
                [
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                ],
                0,
            ),
            optional_field: None,
            default_field: false,
        }),
    );
}

#[test]
fn value_default_fields() {
    assert_eq!(
        TestMessage::builder()
            .group(ux::u4::new(0x2))
            .default_field(true)
            .optional_field(true)
            .build(),
        Ok(TestMessage {
            group: ux::u4::new(0x2),
            data: SliceData(
                [
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                    ux::u7::new(0x0),
                ],
                0,
            ),
            optional_field: Some(true),
            default_field: true,
        }),
    );
}
