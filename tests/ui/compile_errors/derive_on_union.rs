use cloneable_dyn::CloneDyn;

#[derive(CloneDyn)]
union SomeUnion {
    first: i32,
    second: i8,
}

fn main() {}
