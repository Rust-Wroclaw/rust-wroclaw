pub type F = fn(i32) -> i32;

pub fn my_fn(x: i32) -> i32 {
    x * 2
}

pub fn other_fn(x: i32) -> i32 {
    x * 2
}

pub fn run_callback(f: F, x: i32) -> i32 {
    if f == my_fn {
        f(x) + 1
    } else {
        f(x)
    }
}

#[test]
fn what_is_the_result() {
    let a: F = my_fn;
    let b: F = other_fn;

    assert_eq!(a, b);
}
