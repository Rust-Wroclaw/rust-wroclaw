pub fn iterate_over_items(x: &[i32]) {
    let mut iter = x.iter();

    for item in iter.next() {
        println!("{}", item);
    }
}
