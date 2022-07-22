pub fn cast_u8_slice_to_u32_slice(u8_slice: &[u8]) -> &[u32] {
    let u8_pointer = u8_slice as *const [u8];
    let u32_pointer = u8_pointer as *const [u32];

    unsafe { &*u32_pointer }
}
