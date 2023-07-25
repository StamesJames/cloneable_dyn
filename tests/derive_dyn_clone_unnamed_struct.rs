use cloneable_dyn::{dyn_cloneable, dyn_cloneable_for_traits, CloneDyn};

#[dyn_cloneable]
pub trait TestTrait {
    fn incc(&mut self);
    fn get(&self) -> i32;
}
#[derive(CloneDyn)]
struct TestStruct(String, i32, Box<dyn TestTrait>);

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
    let mut original = TestStruct("original".to_string(), 0, Box::new(TestTraitImplementor(0)));
    let mut clone = original.clone();
    assert_eq!(original.0, clone.0);
    assert_eq!(original.1, clone.1);
    clone.0 = "clone".to_string();
    assert_eq!(original.0, "original".to_string());
    assert_eq!(clone.0, "clone".to_string());
    original.0 = "changed".to_string();
    assert_eq!(original.0, "changed".to_string());
    assert_eq!(clone.0, "clone".to_string());
    clone.2.incc();
    assert_eq!(original.2.get(), 0);
    assert_eq!(clone.2.get(), 1);
}
