fn recursive_match(s: &str, p: &str) -> bool {
    if p.is_empty() {
        return s.is_empty()
    }

    if s.is_empty() {
        if p.len() % 2 != 0 {
            return false;
        }
        let mut p_chars = p.chars();
        while let Some(ch) = p_chars.nth(1) {
            if ch != '*' {
                return false;
            }
        }
        return true;
    }

    let mut p_chars = p.chars();
    let p_char = p_chars.next();
    let is_multiple = p_chars.next() == Some('*');
    let s_char = s.chars().next();

    if is_multiple {
        if p_char == s_char || p_char == Some('.') {
            recursive_match(&s[1..], &p) || recursive_match(&s, &p[2..]) || recursive_match(&s[1..], &p[2..]) //TODO: tooo slow
        } else {
            recursive_match(&s, &p[2..])
        }

    } else {
        (p_char == s_char || p_char == Some('.')) && recursive_match(&s[1..], &p[1..])
    }

}

pub fn is_match(s: String, p: String) -> bool {
    recursive_match(&s, &p)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_simple_case() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn should_check_asterisk() {
        assert_eq!(is_match("aa".to_string(), "a*".to_string()), true);
    }

    #[test]
    fn should_check_dot_asterisk() {
        assert_eq!(is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn should_check_asterisk_for_zero() {
        assert_eq!(is_match("aab".to_string(), "c*a*b".to_string()), true);
    }

    #[test]
    fn should_check_not_match() {
        assert_eq!(is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    }

    #[test]
    fn should_check_complex_pattern() {
        assert_eq!(is_match("vldt".to_string(), "va*aldt".to_string()), false);
        assert_eq!(is_match("valdt".to_string(), "va*aldt".to_string()), true);
        assert_eq!(is_match("vaaaaldt".to_string(), "va*aldt".to_string()), true);
    }

    #[test]
    fn should_check_empty_pattern() {
        assert_eq!(is_match("".to_string(), "".to_string()), true);
        assert_eq!(is_match("mississippi".to_string(), "".to_string()), false);
    }

    #[test]
    fn should_check_long_pattern() {
        assert_eq!(is_match("ab".to_string(), ".*c".to_string()), false);
    }

    #[test]
    fn should_check_complex_pattern2() {
        assert_eq!(is_match("aaa".to_string(), "ab*a*c*a".to_string()), true);
    }

    #[test]
    fn should_check_complex_pattern3() {
        assert_eq!(is_match("abacabaca".to_string(), "a.*a*c*a".to_string()), true);
    }

    #[test]
    fn should_check_trailing_pattern() {
        assert_eq!(is_match("a".to_string(), "ab*".to_string()), true);
    }
}