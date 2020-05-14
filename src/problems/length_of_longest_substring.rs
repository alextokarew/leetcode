use std::collections::HashMap;
use std::cmp::min;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut out = 0;
    let mut current = 0;
    let mut indicies = HashMap::new();
    for (index, ch) in s.chars().enumerate() {
        match indicies.insert(ch, index) {
            None => current += 1,
            Some(prev_index) => current = min(index - prev_index, current + 1)
        }
        if current > out {
            out = current
        }
    }
    out as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_correctly() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("pwwkewr".to_string()), 4);
        assert_eq!(length_of_longest_substring("abcbabc".to_string()), 3);
    }
}   