pub fn from_raw_parts_null() -> &'static [u8] {
    unsafe { std::slice::from_raw_parts(std::ptr::null(), 0) }
}
