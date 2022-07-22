/// Returns the last 3 numbers in reversed order.
/// For example `last_3_items_rev(&[1, 2, 3, 4]))` should return `&[4, 3, 2]`.
pub fn last_3_items_rev(x: &[i32]) -> &[i32] {
    let last_3_items = &x[x.len() - 3..];

    &last_3_items[3..0]
}
