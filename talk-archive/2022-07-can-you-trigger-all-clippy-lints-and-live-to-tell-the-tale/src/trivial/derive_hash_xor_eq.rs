#[derive(Hash)]
pub struct MyStrcut;

impl PartialEq for MyStrcut {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
