#[midi2_attr::generate_message]
struct NoOp {
    ump_type: Property<NumericalConstant<0x0>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<0b0000>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;

    #[test]
    fn builder() {
        assert_eq!(
            NoOpOwned::builder().group(u4::new(0xB)).build(),
            Ok(NoOpOwned(arr![0x0B00_0000, 0x0, 0x0, 0x0])),
        )
    }

    #[test]
    fn group() {
        assert_eq!(
            NoOpBorrowed::from_data(&[0x0900_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x9),
        );
    }
}
