pub fn create_a_vector_but_waste_not_a_picosecond() -> Vec<String> {
    let mut v = Vec::with_capacity(1000);

    // Safe because I say so
    unsafe {
        v.set_len(1000);
    }

    v
}
