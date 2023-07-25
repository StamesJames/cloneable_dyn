use cloneable_dyn::{dyn_cloneable, dyn_cloneable_for_traits, CloneDyn};

#[dyn_cloneable]
pub trait TestTrait {
    fn incc(&mut self);
    fn get(&self) -> i32;
}

#[derive(CloneDyn)]
struct TestStruct {
    field0: String,
    field1: i32,
    field2: Box<dyn TestTrait>,
}

#[dyn_cloneable_for_traits(TestTrait)]
#[derive(Clone)]
struct TestTraitImplementor(i32);

impl TestTrait for TestTraitImplementor {
    fn incc(&mut self) {
        self.0 += 1;
    }
    fn get(&self) -> i32 {
        self.0
    }
}

#[test]
pub fn test() {
    let mut original = TestStruct {
        field0: "original".to_string(),
        field1: 0,
        field2: Box::new(TestTraitImplementor(0)),
    };
    let mut clone = original.clone();
    assert_eq!(original.field0, clone.field0);
    assert_eq!(original.field1, clone.field1);
    clone.field0 = "clone".to_string();
    assert_eq!(original.field0, "original".to_string());
    assert_eq!(clone.field0, "clone".to_string());
    original.field0 = "changed".to_string();
    assert_eq!(original.field0, "changed".to_string());
    assert_eq!(clone.field0, "clone".to_string());
    clone.field2.incc();
    assert_eq!(original.field2.get(), 0);
    assert_eq!(clone.field2.get(), 1);
}
