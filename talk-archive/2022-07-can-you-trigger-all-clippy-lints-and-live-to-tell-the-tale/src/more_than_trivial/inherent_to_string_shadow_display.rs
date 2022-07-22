use std::fmt;

pub struct MyStruct;

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct")
    }
}

impl MyStruct {
    // This method shadows the `ToString` one that's auto-derived from the `Display` trait
    pub fn to_string(&self) -> String {
        "MyStruct".to_string()
    }
}
