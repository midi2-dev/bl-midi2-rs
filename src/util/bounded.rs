pub trait Bounded {
    fn absolute_max() -> Self;
    fn absolute_min() -> Self;
}

macro_rules! bounded_impl {
    ($t:ty) => {
        impl Bounded for $t {
            fn absolute_max() -> Self {
                <$t>::MAX
            }
            fn absolute_min() -> Self {
                <$t>::MIN
            }
        }
    };
}

bounded_impl!(ux::u1);
bounded_impl!(ux::u2);
bounded_impl!(ux::u3);
bounded_impl!(ux::u4);
bounded_impl!(ux::u5);
bounded_impl!(ux::u6);
bounded_impl!(ux::u7);
bounded_impl!(u8);
bounded_impl!(ux::u9);
bounded_impl!(ux::u10);
bounded_impl!(ux::u11);
bounded_impl!(ux::u12);
bounded_impl!(ux::u13);
bounded_impl!(ux::u14);
bounded_impl!(ux::u15);
bounded_impl!(u16);
bounded_impl!(ux::u17);
bounded_impl!(ux::u18);
bounded_impl!(ux::u19);
bounded_impl!(ux::u20);
bounded_impl!(ux::u21);
bounded_impl!(ux::u22);
bounded_impl!(ux::u23);
bounded_impl!(ux::u24);
bounded_impl!(ux::u25);
bounded_impl!(ux::u26);
bounded_impl!(ux::u27);
bounded_impl!(ux::u28);
bounded_impl!(ux::u29);
bounded_impl!(ux::u30);
bounded_impl!(ux::u31);
bounded_impl!(u32);
