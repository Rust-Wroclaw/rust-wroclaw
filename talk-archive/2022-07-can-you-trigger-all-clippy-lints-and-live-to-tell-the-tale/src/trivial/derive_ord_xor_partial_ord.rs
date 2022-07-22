use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq)]
pub enum MyEnum {
    A,
    B,
    C,
}

impl Ord for MyEnum {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (x, y) if x == y => Ordering::Equal,
            (MyEnum::A, _) => Ordering::Greater,
            (MyEnum::C, _) => Ordering::Less,
            (MyEnum::B, _) => Ordering::Equal,
        }
    }
}
