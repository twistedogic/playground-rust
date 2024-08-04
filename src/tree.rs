#[allow(dead_code)]
struct KVPair {
    key: u8,
    value: u8,
}

pub trait Mehher {
    fn meh(&self) -> String {
        "what".to_string()
    }
}

impl Mehher for KVPair {
    fn meh(&self) -> String {
        "meh".to_string()
    }
}

pub fn say_meh<T: Mehher>(item: T) -> String {
    item.meh()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_say_meh() {
        let pair = KVPair { key: 1, value: 2 };
        assert_eq!(say_meh(pair), "meh".to_string())
    }
}
