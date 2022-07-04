#[derive(
    Debug, 
    Default, 
    PartialEq,
)]
pub struct Group {}

impl Group {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_group() {
        let _ : Group = Default::default();
    }
}
