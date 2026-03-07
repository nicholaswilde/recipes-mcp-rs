
fn main() {
    let fluff_patterns = [
        r"(?i)^ad:",
        r"(?i)sign up",
        r"(?i)newsletter",
        r"(?i)follow (us|me|on)",
        r"(?i)enjoy",
        r"(?i)rate this",
        r"(?i)instagram",
        r"(?i)facebook",
        r"(?i)pinterest",
        r"(?i)twitter",
        r"(?i)tiktok",
        r"(?i)when I was in",
        r"(?i)grandmother used to",
        r"(?i)first made this",
    ];

    let input = "Enjoy your meal!";
    let mut matched = false;
    for pattern in fluff_patterns {
        let re = regex::Regex::new(pattern).unwrap();
        if re.is_match(input) {
            println!("Matched pattern: {}", pattern);
            matched = true;
            break;
        }
    }
    if !matched {
        println!("No match found");
    }
}
