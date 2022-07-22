use std::mem::ManuallyDrop;

pub struct Stuff;

pub fn drop_my_stuff(stuff: Stuff) {
    let stuff = ManuallyDrop::new(stuff);

    drop(stuff);
}
