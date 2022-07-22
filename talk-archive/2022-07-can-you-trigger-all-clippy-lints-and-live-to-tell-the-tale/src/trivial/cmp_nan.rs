pub fn compare_to_nan(x: f32) -> bool {
    x == f32::NAN
}

// Fun fact:
// the error message from Clippy says this comparison is "doomed"
// ```
// error: doomed comparison with `NAN`, use `{f32,f64}::is_nan()` instead
// ```
