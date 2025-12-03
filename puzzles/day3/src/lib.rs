use std::fmt::Display;

pub fn digits_into_usize<T: Display>(digits: Vec<T>) -> Result<usize, std::num::ParseIntError> {
    digits
        .iter()
        .fold(String::new(), |acc, d| format!("{}{}", acc, d))
        .parse::<usize>()
}

/// Find the maximum value and its index
pub fn index_of_max<T: Iterator<Item = u32>>(digits: T) -> Option<(usize, u32)> {
    digits
        .enumerate()
        .reduce(|(i1, d1), (i2, d2)| if d2 > d1 { (i2, d2) } else { (i1, d1) })
}

/// `digits` is the number of remaining digits to extract from `v`
pub fn extract_max_digits(v: Vec<u32>, digits: usize) -> Vec<u32> {
    if digits == 0 {
        vec![]
    } else {
        let usable = v.clone().into_iter().take(v.len() - digits + 1);
        let (i, next_max) = index_of_max(usable).unwrap();
        let rest = v.into_iter().skip(i + 1).collect();
        [vec![next_max], extract_max_digits(rest, digits - 1)].concat()
    }
}
