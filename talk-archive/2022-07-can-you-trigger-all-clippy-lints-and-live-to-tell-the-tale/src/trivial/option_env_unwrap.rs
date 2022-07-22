pub fn get_home_dir() -> &'static str {
    option_env!("HOME").unwrap()
}
