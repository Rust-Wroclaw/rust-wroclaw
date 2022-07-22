pub fn drop_my_reference<T>(x: &T) {
    drop(x);
}
