use crate::group::Group;
use crate::group_id::GroupID;

pub struct Context {
    groups: [Option<Group>; 16],
}

impl Context {
    pub fn new() -> Context {
       Context{
           groups: Default::default(),
       }
    }

    pub fn groups(&self) -> &[Option<Group>; 16] {
        &self.groups
    }

    pub fn group(&self, id: GroupID) -> Option<&Group> {
        self.groups[id as usize].as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_context() {
        let _context = Context::new();
    }

    #[test]
    fn retreive_groups() {
        let context = Context::new();
        let expected: [Option<Group>; 16] = Default::default();
        assert_eq!(context.groups(), &expected);
    }

    #[test]
    fn retreive_group() {
        let context = Context::new();
        assert_eq!(context.group(GroupID::G1), None);
    }
}
