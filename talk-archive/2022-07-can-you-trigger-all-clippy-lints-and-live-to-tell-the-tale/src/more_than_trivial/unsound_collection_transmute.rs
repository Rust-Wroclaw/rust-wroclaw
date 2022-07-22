pub fn quick_convert(v: Vec<i32>) -> Vec<i16> {
    unsafe { std::mem::transmute(v) }
}
