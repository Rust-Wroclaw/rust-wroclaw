pub enum Void {}

pub fn increment_empty_tuple_pointer(ptr: *mut Void) {
    unsafe {
        let _reference: &Void = &*ptr.offset(1);
    }
}

// godbolt.org emits the following assembly
// ```
// example::increment_empty_tuple_pointer:
//         ret
// ```
