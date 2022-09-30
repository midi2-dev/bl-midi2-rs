use midi2_derive::Builder;
use crate::error::Error;

#[derive(Builder)]
#[derive(PartialEq, Debug)]
pub struct TestMessage {
    group: ux::u4,
    data: [ux::u7; 4],
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
            .data(Default::default())
            .optional_field(true)
            .default_field(true)
            .build(),
        Ok(TestMessage {
            group: ux::u4::new(0xA),
            data: [
                ux::u7::new(0x0),
                ux::u7::new(0x0),
                ux::u7::new(0x0),
                ux::u7::new(0x0),
            ],
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
            .data(Default::default())
            .default_field(true)
            .build(),
        Ok(TestMessage {
            group: ux::u4::new(0x2),
            data: [
                ux::u7::new(0x0),
                ux::u7::new(0x0),
                ux::u7::new(0x0),
                ux::u7::new(0x0),
            ],
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
            .data(Default::default())
            .build(),
        Ok(TestMessage {
            group: ux::u4::new(0x2),
            data: [
                ux::u7::new(0x0),
                ux::u7::new(0x0),
                ux::u7::new(0x0),
                ux::u7::new(0x0),
            ],
            optional_field: None,
            default_field: false,
        }),
    );
}
