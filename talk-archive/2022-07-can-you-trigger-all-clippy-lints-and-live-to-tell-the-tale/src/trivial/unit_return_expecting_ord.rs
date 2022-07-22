pub fn oldest_person(v: &[(String, u32)]) -> &str {
    let oldest = v.iter().max_by_key(|x| {
        x.1;
    });

    oldest.unwrap().0.as_str()
}
