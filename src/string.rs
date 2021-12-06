pub fn is_same_number(want: &str, got: u8) -> bool {
    let s = String::from(want)
        .trim()
        .parse::<u8>()
        .expect("want is not a number");
    s == got
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_same_number() {
        assert!(is_same_number("2", 2));
    }
}
