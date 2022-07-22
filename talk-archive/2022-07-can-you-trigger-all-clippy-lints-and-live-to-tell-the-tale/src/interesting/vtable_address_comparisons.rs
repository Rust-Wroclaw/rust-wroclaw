use std::rc::Rc;

pub trait MyTrait {
    fn echo(&self);
}

impl MyTrait for () {
    fn echo(&self) {
        println!("()");
    }
}

impl MyTrait for u32 {
    fn echo(&self) {
        println!("{self}");
    }
}

pub fn compare_items(a: Rc<dyn MyTrait>, b: Rc<dyn MyTrait>) -> bool {
    Rc::ptr_eq(&a, &b)
}

#[test]
fn are_these_two_very_different_items_the_same() {
    let a: Rc<dyn MyTrait> = Rc::new(());
    let b: Rc<dyn MyTrait> = Rc::new(());

    assert!(compare_items(a, b));
}
