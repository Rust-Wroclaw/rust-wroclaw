pub fn swap<T: Copy>(a: &mut T, b: &mut T) {
    *a = *b;
    *b = *a;
}
