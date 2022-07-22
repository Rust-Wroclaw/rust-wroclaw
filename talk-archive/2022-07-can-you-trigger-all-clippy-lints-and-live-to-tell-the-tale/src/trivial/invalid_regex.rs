use regex::Regex;

pub fn contains_stuff(text: &str) -> bool {
    let regex = Regex::new("(").unwrap();

    regex.is_match(text)
}

// Fun fact: The example provided on the clippy website is `Regex::new("|")` which actually isn't an incorrect regex
