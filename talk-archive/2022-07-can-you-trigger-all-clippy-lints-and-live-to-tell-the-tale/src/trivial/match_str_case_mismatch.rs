pub fn is_quit_command(cmd: &str) -> bool {
    match cmd.to_lowercase().as_str() {
        "QUIT" => true,
        "Q" => true,
        "EXIT" => true,
        _ => false,
    }
}
