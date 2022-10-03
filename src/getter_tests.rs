use getters_derive::Getters;

#[derive(Getters)]
#[derive(Default, Debug, PartialEq)]
struct TestMessage {
    note: ux::u7,
    #[getters(ref)]
    id: [u8; 3],
    #[getters(slice_interface)]
    data: ([ux::u7; 10], usize),
}

#[test]
fn getter() {
    assert_eq!(
        TestMessage { note: ux::u7::new(0x50), ..Default::default() }.note(),
        ux::u7::new(0x50),
    );
}

#[test]
fn ref_getter() {
    assert_eq!(
        TestMessage { id: [0x1, 0x2, 0x3], ..Default::default() }.id(),
        &[0x1, 0x2, 0x3],
    );
}

#[test]
fn slice_interface_getter() {
    assert_eq!(
        TestMessage { 
            data: (
                [
                    ux::u7::new(0x1),
                    ux::u7::new(0x2),
                    ux::u7::new(0x3),
                    ux::u7::new(0x4),
                    ux::u7::new(0x5),
                    ux::u7::new(0x6),
                    ux::u7::new(0x7),
                    ux::u7::new(0x8),
                    ux::u7::new(0x9),
                    ux::u7::new(0xA),
                ],
                4,
            ), 
            ..Default::default() 
        }.data(),
        &[
            ux::u7::new(0x1), 
            ux::u7::new(0x2), 
            ux::u7::new(0x3), 
            ux::u7::new(0x4)
        ],
    );
}
