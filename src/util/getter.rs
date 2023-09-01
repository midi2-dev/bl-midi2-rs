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
        group: u4,
        note: u7,
    }

    impl Message {
        getter!(group, u4);
        getter!(note, u7);
    }

    #[test]
    fn call_getter() {
        let m = Message {
            group: u4::new(0xA),
            note: u7::new(0x7F),
        };
        assert_eq!(m.group(), u4::new(0xA));
        assert_eq!(m.note(), u7::new(0x7F));
    }
}
