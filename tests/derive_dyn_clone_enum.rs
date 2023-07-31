use cloneable_dyn::{dyn_cloneable, dyn_cloneable_for_traits, CloneDyn};

#[dyn_cloneable]
pub trait TestTrait {
    fn incc(&mut self);
    fn get(&self) -> i32;
}

#[derive(CloneDyn)]
enum TestEnum {
    Named {
        field0: String,
        field1: i32,
        field2: Box<dyn TestTrait>,
    },
    Unnamed(String, i32, Box<dyn TestTrait>),
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
    let original = TestEnum::Named {
        field0: "original".to_string(),
        field1: 0,
        field2: Box::new(TestTraitImplementor(0)),
    };
    let clone = original.clone();
    match (original, clone) {
        (
            TestEnum::Named {
                mut field0,
                field1,
                field2,
            },
            TestEnum::Named {
                field0: mut field0clone,
                field1: field1clone,
                field2: mut field2clone,
            },
        ) => {
            assert_eq!(field0, field0clone);
            assert_eq!(field1, field1clone);
            field0clone = "clone".to_string();
            assert_eq!(field0, "original".to_string());
            assert_eq!(field0clone, "clone".to_string());
            field0 = "changed".to_string();
            assert_eq!(field0, "changed".to_string());
            assert_eq!(field0clone, "clone".to_string());
            field2clone.incc();
            assert_eq!(field2.get(), 0);
            assert_eq!(field2clone.get(), 1);
        }
        _ => panic!("should not be reached"),
    }

    let original = TestEnum::Unnamed("original".to_string(), 0, Box::new(TestTraitImplementor(0)));
    let clone = original.clone();
    match (original, clone) {
        (
            TestEnum::Unnamed(mut field0, field1, field2),
            TestEnum::Unnamed(mut field0clone, field1clone, mut field2clone),
        ) => {
            assert_eq!(field0, field0clone);
            assert_eq!(field1, field1clone);
            field0clone = "clone".to_string();
            assert_eq!(field0, "original".to_string());
            assert_eq!(field0clone, "clone".to_string());
            field0 = "changed".to_string();
            assert_eq!(field0, "changed".to_string());
            assert_eq!(field0clone, "clone".to_string());
            field2clone.incc();
            assert_eq!(field2.get(), 0);
            assert_eq!(field2clone.get(), 1);
        }
        _ => panic!("should not be reached"),
    }
}
