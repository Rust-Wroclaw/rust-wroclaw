#[allow(invalid_value)]

pub fn reinitialize_vec(v: &mut Vec<i32>) {
    let taken_v = unsafe { std::mem::replace(v, std::mem::zeroed()) };

    let invalid_uninit = std::mem::replace(v, taken_v);

    std::mem::forget(invalid_uninit);
}
