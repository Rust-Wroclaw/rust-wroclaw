use std::mem;

pub fn null_u64() -> *const u64 {
    unsafe { mem::transmute(0 as *const u64) }
}
