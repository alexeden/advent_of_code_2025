/// "1-10" -> 1..=10
pub fn parse_range_or_panic(s: &str) -> std::ops::RangeInclusive<usize> {
    if let Some((Ok(a), Ok(b))) = s
        .split_once('-')
        .map(|(a, b)| (a.parse::<usize>(), b.parse::<usize>()))
    {
        a..=b
    } else {
        panic!("Invalid range: {s}");
    }
}

pub fn has_repeating_halves(s: &str) -> bool {
    if !s.len().is_multiple_of(2) {
        false
    } else {
        let (a, b) = s.split_at(s.len() / 2);
        a == b
    }
}

/// Split a string into a vector of chunks of size `n`
pub fn chunkify(s: &str, n: usize) -> Vec<&str> {
    if s.len() <= n {
        vec![s]
    } else {
        let (a, rest) = s.split_at(n);
        [vec![a], chunkify(rest, n)].concat()
    }
}

/// Run a non-overlapping sliding window over the string, and check if for any
/// frame size, all frame values are the same
pub fn has_repeating_substring(s: &str) -> bool {
    (1..=s.len() / 2)
        .filter(|n| s.len().is_multiple_of(*n)) // smol optimization
        .map(|n| chunkify(s, n))
        .any(|chunks| chunks.iter().all(|c| c == &chunks[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_repeating_substring() {
        assert!(has_repeating_substring("22"));
        assert!(has_repeating_substring("111"));
        assert!(has_repeating_substring("1010"));
        assert!(has_repeating_substring("1188511885"));
        assert!(has_repeating_substring("212121212121"));
        assert!(has_repeating_substring("446446"));
    }

    #[test]
    fn test_chunkify() {
        assert_eq!(chunkify("", 10), vec![""]);
        assert_eq!(
            chunkify("1234567890", 1),
            vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]
        );
        assert_eq!(chunkify("1234567890", 3), vec!["123", "456", "789", "0"]);
        assert_eq!(chunkify("1234567890", 4), vec!["1234", "5678", "90"]);
    }
}
