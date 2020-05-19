pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();

    if s.len() < 2 {
        return s
    }

    let mut i = 1;
    let mut from = 0;
    let mut to = 1;

    while i < chars.len() - (to - from) / 2 {
        if chars[i - 1] == chars[i] {
            expand(i - 1, i, &mut from, &mut to, &chars);
        }
        if i < chars.len() - 1 && chars[i - 1] == chars[i + 1] {
            expand(i - 1, i + 1, &mut from, &mut to, &chars);
        }
        i += 1
    }

    chars[from..to].to_vec().iter().collect::<String>()
}

fn expand(mut from_new: usize, mut to_new: usize, from: &mut usize, to: &mut usize, chars: &Vec<char>) {
    while from_new > 0 && to_new < chars.len() - 1 && chars[from_new - 1] == chars[to_new + 1] {
        from_new -= 1;
        to_new += 1;
    }

    to_new += 1;

    if to_new - from_new > *to - *from {
        *from = from_new;
        *to = to_new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_longest_palindrome() {
        assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string())
    }
    
    #[test]
    fn should_return_empty_string_when_input_is_empty() {
        assert_eq!(longest_palindrome("".to_string()), "".to_string())
    }

    #[test]
    fn should_return_self_when_input_is_single_character() {
        assert_eq!(longest_palindrome("x".to_string()), "x".to_string());
        assert_eq!(longest_palindrome("Ы".to_string()), "Ы".to_string())
    }

    #[test]
    fn should_return_first_char_when_there_is_no_palindrome_in_it() {
        assert_eq!(longest_palindrome("abcdef".to_string()), "a".to_string())
    }

    #[test]
    fn should_deal_with_embedded_palindromes() {
        assert_eq!(longest_palindrome("abahabapravitmirom".to_string()), "abahaba".to_string())
    }

    #[test]
    fn should_return_self_for_equally_chars_string() {
        assert_eq!(longest_palindrome("bbbbbbb".to_string()), "bbbbbbb".to_string());
        assert_eq!(longest_palindrome("bbbbbbbb".to_string()), "bbbbbbbb".to_string())
    }

    #[test]
    fn should_deal_with_last_place_case() {
        assert_eq!(longest_palindrome("abcdee".to_string()), "ee".to_string())
    }

    #[test]
    fn should_deal_when_the_middle_is_three_euqal_chars() {
        assert_eq!(longest_palindrome("trabbbartex".to_string()), "trabbbart".to_string())
    }
}