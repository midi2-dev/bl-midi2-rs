pub(crate) trait Center:
    Into<u32> + TryFrom<u32> + Sized + Copy + PartialEq + PartialOrd
where
    <Self as TryFrom<u32>>::Error: core::fmt::Debug,
{
    const MIN: Self;
    const MAX: Self;

    fn center_value() -> Self {
        let min: u32 = Self::MIN.into();
        let max: u32 = Self::MAX.into();

        ((max - min) / 2_u32 + 1)
            .try_into()
            .expect("Center shouldn't be larger than max.")
    }
}

impl Center for ux::u7 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for ux::u9 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for ux::u14 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for u16 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

impl Center for u32 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}

pub(crate) trait MinCenterMax: Center + core::ops::Shr<u32>
where
    <Self as core::ops::Shr<u32>>::Output: Into<Self>,
    <Self as TryFrom<u32>>::Error: core::fmt::Debug,
{
    #[allow(dead_code)]
    fn upscale<
        U: Center
            + core::ops::Add<Output = U>
            + core::ops::BitAnd<Output = U>
            + core::fmt::Debug
            + core::ops::BitOr<Output = U>
            + core::ops::Shr<u32, Output = U>
            + core::ops::Shl<u32, Output = U>,
    >(
        self,
    ) -> U
    where
        Self: Into<U>,
        <U as TryFrom<u32>>::Error: core::fmt::Debug,
    {
        let min = Self::MIN;
        let center = Self::center_value();
        let max = Self::MAX;

        match self {
            s if s == min => U::MIN,
            s if s == max => U::MAX,
            s if s == center => U::center_value(),

            s if (min..center).contains(&s) => {
                let self_max: u32 = Self::MAX.into();
                let other_max: u32 = U::MAX.into();
                let shift = (other_max - self_max).count_ones();
                let other: U = self.into();
                other << shift
            }

            s if (center..max).contains(&s) => {
                let self_max: u32 = Self::MAX.into();
                let other_max: u32 = U::MAX.into();
                let self_bits = self_max.count_ones();
                let other_bits = other_max.count_ones();
                let shift = self_bits - 1;

                let initial: U = self.into();
                let initial_mask = U::MAX >> (other_bits - self_bits + 1);
                let repeating = initial & initial_mask;

                let mut upscaled = initial;
                let mut remainder = other_bits - self_bits;

                while remainder > 0 {
                    remainder = match remainder {
                        remainder if remainder > shift => {
                            upscaled = (upscaled << shift) | repeating;
                            remainder - shift
                        }
                        _ => {
                            let shift = shift - remainder;
                            upscaled = (upscaled << remainder) | (repeating >> shift);
                            0
                        }
                    };
                }
                upscaled
            }

            _ => self.into(),
        }
    }

    fn downscale<U: Center + TryFrom<Self>>(self) -> U
    where
        <U as TryFrom<u32>>::Error: core::fmt::Debug,
        <U as TryFrom<Self>>::Error: core::fmt::Debug,
    {
        let self_max: u32 = Self::MAX.into();
        let other_max: u32 = U::MAX.into();
        let shift = (self_max - other_max).count_ones();
        let downscaled = self >> shift;
        let downscaled: Self = downscaled.into();

        downscaled.try_into().expect("Downscaling should not fail.")
    }
}

impl<U: Center + core::ops::Shr<u32>> MinCenterMax for U
where
    <U as TryFrom<u32>>::Error: core::fmt::Debug,
    <U as core::ops::Shr<u32>>::Output: Into<U>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mins_upscaling() {
        let min_u7 = ux::u7::new(0);
        let min_u9 = ux::u9::new(0);
        let min_u14 = ux::u14::new(0);
        let min_u16 = 0_u16;
        let min_u32 = 0_u32;

        assert_eq!(min_u7.upscale::<ux::u7>(), min_u7);
        assert_eq!(min_u7.upscale::<ux::u9>(), min_u9);
        assert_eq!(min_u7.upscale::<ux::u14>(), min_u14);
        assert_eq!(min_u7.upscale::<u16>(), min_u16);
        assert_eq!(min_u7.upscale::<u32>(), min_u32);

        assert_eq!(min_u9.upscale::<ux::u9>(), min_u9);
        assert_eq!(min_u9.upscale::<ux::u14>(), min_u14);
        assert_eq!(min_u9.upscale::<u16>(), min_u16);
        assert_eq!(min_u9.upscale::<u32>(), min_u32);

        assert_eq!(min_u14.upscale::<ux::u14>(), min_u14);
        assert_eq!(min_u14.upscale::<u16>(), min_u16);
        assert_eq!(min_u14.upscale::<u32>(), min_u32);

        assert_eq!(min_u16.upscale::<u16>(), min_u16);
        assert_eq!(min_u16.upscale::<u32>(), min_u32);

        assert_eq!(min_u32.upscale::<u32>(), min_u32);
    }

    #[test]
    fn test_maxs_upscaling() {
        let max_u7 = ux::u7::new(127);
        let max_u9 = ux::u9::new(511);
        let max_u14 = ux::u14::new(16383);
        let max_u16 = 0xFFFF_u16;
        let max_u32 = 0xFFFFFFFF_u32;

        assert_eq!(max_u7.upscale::<ux::u7>(), max_u7);
        assert_eq!(max_u7.upscale::<ux::u9>(), max_u9);
        assert_eq!(max_u7.upscale::<ux::u14>(), max_u14);
        assert_eq!(max_u7.upscale::<u16>(), max_u16);
        assert_eq!(max_u7.upscale::<u32>(), max_u32);

        assert_eq!(max_u9.upscale::<ux::u9>(), max_u9);
        assert_eq!(max_u9.upscale::<ux::u14>(), max_u14);
        assert_eq!(max_u9.upscale::<u16>(), max_u16);
        assert_eq!(max_u9.upscale::<u32>(), max_u32);

        assert_eq!(max_u14.upscale::<ux::u14>(), max_u14);
        assert_eq!(max_u14.upscale::<u16>(), max_u16);
        assert_eq!(max_u14.upscale::<u32>(), max_u32);

        assert_eq!(max_u16.upscale::<u16>(), max_u16);
        assert_eq!(max_u16.upscale::<u32>(), max_u32);

        assert_eq!(max_u32.upscale::<u32>(), max_u32);
    }

    #[test]
    fn test_centers_upscaling() {
        let center_u7 = ux::u7::new(0x40);
        let center_u9 = ux::u9::new(256);
        let center_u14 = ux::u14::new(8192);
        let center_u16 = 0x8000_u16;
        let center_u32 = 0x80000000_u32;

        assert_eq!(center_u7.upscale::<ux::u7>(), center_u7);
        assert_eq!(center_u7.upscale::<ux::u9>(), center_u9);
        assert_eq!(center_u7.upscale::<ux::u14>(), center_u14);
        assert_eq!(center_u7.upscale::<u16>(), center_u16);
        assert_eq!(center_u7.upscale::<u32>(), center_u32);

        assert_eq!(center_u9.upscale::<ux::u9>(), center_u9);
        assert_eq!(center_u9.upscale::<ux::u14>(), center_u14);
        assert_eq!(center_u9.upscale::<u16>(), center_u16);
        assert_eq!(center_u9.upscale::<u32>(), center_u32);

        assert_eq!(center_u14.upscale::<ux::u14>(), center_u14);
        assert_eq!(center_u14.upscale::<u16>(), center_u16);
        assert_eq!(center_u14.upscale::<u32>(), center_u32);

        assert_eq!(center_u16.upscale::<u16>(), center_u16);
        assert_eq!(center_u16.upscale::<u32>(), center_u32);

        assert_eq!(center_u32.upscale::<u32>(), center_u32);
    }

    #[test]
    fn test_lower_range_upscaling() {
        let lower_u7 = ux::u7::new(0x1F);
        let lower_u9 = ux::u9::new(0x7C);
        let lower_u14 = ux::u14::new(0xF80);
        let lower_u16 = 0x3E00_u16;
        let lower_u32 = 0x3E00_0000_u32;

        assert_eq!(lower_u7.upscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u7.upscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u7.upscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u7.upscale::<u16>(), lower_u16);
        assert_eq!(lower_u7.upscale::<u32>(), lower_u32);

        assert_eq!(lower_u9.upscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u9.upscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u9.upscale::<u16>(), lower_u16);
        assert_eq!(lower_u9.upscale::<u32>(), lower_u32);

        assert_eq!(lower_u14.upscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u14.upscale::<u16>(), lower_u16);
        assert_eq!(lower_u14.upscale::<u32>(), lower_u32);

        assert_eq!(lower_u16.upscale::<u16>(), lower_u16);
        assert_eq!(lower_u16.upscale::<u32>(), lower_u32);

        assert_eq!(lower_u32.upscale::<u32>(), lower_u32);
    }

    #[test]
    fn test_upper_range_upscaling() {
        // Test cases from MIDI 2.0 Bit Scaling and Resolution v1.0.2
        let upper_u7_to_u16 = [
            (ux::u7::new(0x46).upscale::<u16>(), 0x8C30_u16),
            (ux::u7::new(0x60).upscale::<u16>(), 0xC104_u16),
            (ux::u7::new(0x78).upscale::<u16>(), 0xF1C7_u16),
        ];

        let upper_u7_to_u32 = [
            (ux::u7::new(0x46).upscale::<u32>(), 0x8C30C30C_u32),
            (ux::u7::new(0x60).upscale::<u32>(), 0xC1041041_u32),
            (ux::u7::new(0x78).upscale::<u32>(), 0xF1C71C71_u32),
        ];

        let upper_u16_to_u32 = [
            (0x9C40_u16.upscale::<u32>(), 0x9C403880_u32),
            (0xC000_u16.upscale::<u32>(), 0xC0008001_u32),
            (0xFDE8_u16.upscale::<u32>(), 0xFDE8FBD1_u32),
        ];

        for (result, expected) in upper_u7_to_u16 {
            assert_eq!(result, expected);
        }

        for (result, expected) in upper_u7_to_u32 {
            assert_eq!(result, expected);
        }

        for (result, expected) in upper_u16_to_u32 {
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_lower_range_downscaling() {
        let lower_u7 = ux::u7::new(31);
        let lower_u9 = ux::u9::new(127);
        let lower_u14 = ux::u14::new(4095);
        let lower_u16 = 0x3FFF_u16;
        let lower_u32 = 0x3FFFFFFF_u32;

        assert_eq!(lower_u32.downscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u32.downscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u32.downscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u32.downscale::<u16>(), lower_u16);
        assert_eq!(lower_u32.downscale::<u32>(), lower_u32);

        assert_eq!(lower_u16.downscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u16.downscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u16.downscale::<ux::u14>(), lower_u14);
        assert_eq!(lower_u16.downscale::<u16>(), lower_u16);

        assert_eq!(lower_u14.downscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u14.downscale::<ux::u9>(), lower_u9);
        assert_eq!(lower_u14.downscale::<ux::u14>(), lower_u14);

        assert_eq!(lower_u9.downscale::<ux::u7>(), lower_u7);
        assert_eq!(lower_u9.downscale::<ux::u9>(), lower_u9);

        assert_eq!(lower_u7.downscale::<ux::u7>(), lower_u7);
    }

    #[test]
    fn test_centers_downscaling() {
        let center_u7 = ux::u7::new(0x40);
        let center_u9 = ux::u9::new(256);
        let center_u14 = ux::u14::new(8192);
        let center_u16 = 0x8000_u16;
        let center_u32 = 0x80000000_u32;

        assert_eq!(center_u32.downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u32.downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u32.downscale::<ux::u14>(), center_u14);
        assert_eq!(center_u32.downscale::<u16>(), center_u16);
        assert_eq!(center_u32.downscale::<u32>(), center_u32);

        assert_eq!(center_u16.downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u16.downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u16.downscale::<ux::u14>(), center_u14);
        assert_eq!(center_u16.downscale::<u16>(), center_u16);

        assert_eq!(center_u14.downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u14.downscale::<ux::u9>(), center_u9);
        assert_eq!(center_u14.downscale::<ux::u14>(), center_u14);

        assert_eq!(center_u9.downscale::<ux::u7>(), center_u7);
        assert_eq!(center_u9.downscale::<ux::u9>(), center_u9);

        assert_eq!(center_u7.downscale::<ux::u7>(), center_u7);
    }

    #[test]
    fn test_mins_downscaling() {
        let min_u7 = ux::u7::new(0);
        let min_u9 = ux::u9::new(0);
        let min_u14 = ux::u14::new(0);
        let min_u16 = 0_u16;
        let min_u32 = 0_u32;

        assert_eq!(min_u32.downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u32.downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u32.downscale::<ux::u14>(), min_u14);
        assert_eq!(min_u32.downscale::<u16>(), min_u16);
        assert_eq!(min_u32.downscale::<u32>(), min_u32);

        assert_eq!(min_u16.downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u16.downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u16.downscale::<ux::u14>(), min_u14);
        assert_eq!(min_u16.downscale::<u16>(), min_u16);

        assert_eq!(min_u14.downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u14.downscale::<ux::u9>(), min_u9);
        assert_eq!(min_u14.downscale::<ux::u14>(), min_u14);

        assert_eq!(min_u9.downscale::<ux::u7>(), min_u7);
        assert_eq!(min_u9.downscale::<ux::u9>(), min_u9);

        assert_eq!(min_u7.downscale::<ux::u7>(), min_u7);
    }

    #[test]
    fn test_maxs_downscaling() {
        let max_u7 = ux::u7::new(127);
        let max_u9 = ux::u9::new(511);
        let max_u14 = ux::u14::new(16383);
        let max_u16 = 0xFFFF_u16;
        let max_u32 = 0xFFFFFFFF_u32;

        assert_eq!(max_u32.downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u32.downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u32.downscale::<ux::u14>(), max_u14);
        assert_eq!(max_u32.downscale::<u16>(), max_u16);
        assert_eq!(max_u32.downscale::<u32>(), max_u32);

        assert_eq!(max_u16.downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u16.downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u16.downscale::<ux::u14>(), max_u14);
        assert_eq!(max_u16.downscale::<u16>(), max_u16);

        assert_eq!(max_u14.downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u14.downscale::<ux::u9>(), max_u9);
        assert_eq!(max_u14.downscale::<ux::u14>(), max_u14);

        assert_eq!(max_u9.downscale::<ux::u7>(), max_u7);
        assert_eq!(max_u9.downscale::<ux::u9>(), max_u9);

        assert_eq!(max_u7.downscale::<ux::u7>(), max_u7);
    }

    #[test]
    fn test_example_downscaling() {
        let examples = [
            (5120_u16.downscale::<ux::u7>(), ux::u7::new(10)),
            (32768_u16.downscale::<ux::u7>(), ux::u7::new(64)),
            (44730_u16.downscale::<ux::u7>(), ux::u7::new(87)),
            (65535_u16.downscale::<ux::u7>(), ux::u7::new(127)),
        ];

        for (input, expected) in examples {
            assert_eq!(input, expected);
        }
    }
}
