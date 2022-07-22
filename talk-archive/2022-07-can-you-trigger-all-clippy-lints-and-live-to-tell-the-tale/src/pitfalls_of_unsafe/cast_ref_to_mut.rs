pub fn mutate_immutable(x: &i32) {
    let x: &mut i32 = unsafe { &mut *(x as *const _ as *mut _) };

    *x += 1;
}
