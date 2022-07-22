pub fn parse_identifier(s: &str) -> [u32; 2] {
    let mut parts = s.splitn(1, ':');

    let first = parts.next().unwrap();
    let second = parts.next().unwrap();

    [
        first.parse::<u32>().unwrap(),
        second.parse::<u32>().unwrap(),
    ]
}
