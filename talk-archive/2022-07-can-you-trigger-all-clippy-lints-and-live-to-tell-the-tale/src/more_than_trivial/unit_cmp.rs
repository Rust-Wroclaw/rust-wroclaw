pub fn compare_items<T, F>(a: T, b: T, f: F) -> bool
where
    F: Fn(T) -> i32,
{
    let a_val = {
        f(a);
    };
    let b_val = {
        f(b);
    };

    a_val == b_val
}
