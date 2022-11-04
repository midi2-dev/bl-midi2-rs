macro_rules! getter {
    ($member:ident, $t:ty) => {
        pub fn $member(&self) -> $t {
            self.$member
        }
    };
}

pub(crate) use getter;

#[cfg(test)]
mod tests {
    use super::getter;

    struct Message {
        group: ux::u4,
        note: ux::u7,
    }

    impl Message {
        getter!(group, ux::u4);
        getter!(note, ux::u7);
    }

    #[test]
    fn call_getter() {
        let m = Message {
            group: ux::u4::new(0xA),
            note: ux::u7::new(0x7F),
        };
        assert_eq!(m.group(), ux::u4::new(0xA));
        assert_eq!(m.note(), ux::u7::new(0x7F));
    }
}
