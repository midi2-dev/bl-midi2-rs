use ux::*;

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

bounded_impl!(u1);
bounded_impl!(u2);
bounded_impl!(u3);
bounded_impl!(u4);
bounded_impl!(u5);
bounded_impl!(u6);
bounded_impl!(u7);
bounded_impl!(u8);
bounded_impl!(u9);
bounded_impl!(u10);
bounded_impl!(u11);
bounded_impl!(u12);
bounded_impl!(u13);
bounded_impl!(u14);
bounded_impl!(u15);
bounded_impl!(u16);
bounded_impl!(u17);
bounded_impl!(u18);
bounded_impl!(u19);
bounded_impl!(u20);
bounded_impl!(u21);
bounded_impl!(u22);
bounded_impl!(u23);
bounded_impl!(u24);
bounded_impl!(u25);
bounded_impl!(u26);
bounded_impl!(u27);
bounded_impl!(u28);
bounded_impl!(u29);
bounded_impl!(u30);
bounded_impl!(u31);
bounded_impl!(u32);
