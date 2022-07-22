pub fn maximum(items: &[u32]) -> u32 {
    let max_item = items.iter().max().copied();

    if max_item.is_none() {
        max_item.unwrap()
    } else {
        0
    }
}
