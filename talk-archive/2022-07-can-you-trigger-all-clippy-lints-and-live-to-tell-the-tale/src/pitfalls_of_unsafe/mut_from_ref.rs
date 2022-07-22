#![allow(unused_unsafe)]

pub fn convert_ref_to_mut(_x: &i32) -> &mut i32 {
    panic!("Very illegal, please stop")
}

pub fn convert_ref_to_mut_but_with_unsafe(_x: &i32) -> &mut i32 {
    unsafe { panic!("Very illegal and also unsafe, please stop") }
}
