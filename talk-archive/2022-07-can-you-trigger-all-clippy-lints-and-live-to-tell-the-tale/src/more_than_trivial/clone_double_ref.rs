#![allow(unused)]

pub fn clone_double_ref() {
    let x: Vec<i32> = vec![];

    let ref_to_x = &x;
    let double_ref_to_x = &&x;

    // Calling clone on `x` or `ref_to_x` yields the same result due to auto dereferencing.
    let x_cloned: Vec<i32> = x.clone();
    let another_x_cloned: Vec<i32> = ref_to_x.clone();

    // The resulting values are actual clones with different pointers
    assert_ne!(&x as *const _, &x_cloned as *const _);
    assert_ne!(&x as *const _, &another_x_cloned as *const _);

    // Calling clone on `double_ref_to_x` clones the pointer itself
    let ref_to_x_cloned: &Vec<i32> = double_ref_to_x.clone();

    // The resulting value is the same pointer
    assert_eq!(&x as *const _, ref_to_x_cloned as *const _);
}
