use std::intrinsics::copy_nonoverlapping;
use std::mem::size_of;

const SIZE: usize = 128;

pub fn copy(src: &[u32], dst: &mut [u32]) {
    unsafe {
        copy_nonoverlapping(
            src.as_ptr(),
            dst.as_mut_ptr(),
            size_of::<u32>() * SIZE,
        )
    }
}
