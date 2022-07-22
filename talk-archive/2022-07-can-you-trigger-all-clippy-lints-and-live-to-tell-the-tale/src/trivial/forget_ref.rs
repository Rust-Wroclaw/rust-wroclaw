pub fn forget_my_reference<T>(x: &T) {
    std::mem::forget(x);
}
