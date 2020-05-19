pub fn my_atoi(str: String) -> i32 {
    let mut out = 0;
    let mut whitespaces_flag = true;
    let mut sign = 1;

    let min_bound = std::i32::MIN / 10;
    let max_bound = std::i32::MAX / 10;

    for ch in str.chars() {
        if '0' <= ch && ch <= '9' {
            whitespaces_flag = false;
            let delta = sign*(ch as i32 - '0' as i32);

            if out < 0 && (out < min_bound || std::i32::MIN - out*10 > delta) {
                return std::i32::MIN;
            }

            if out > 0 && (out > max_bound || std::i32::MAX - out*10 < delta) {
                return std::i32::MAX;
            }

            out = out * 10 + delta;
        } else if whitespaces_flag && ch == '-' {
            whitespaces_flag = false;
            sign = -1;
        } else if whitespaces_flag && ch == '+' {
            whitespaces_flag = false;
        } else if whitespaces_flag && ch == ' ' {
            //No op
        } else {
            return out;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_positive_number() {
        assert_eq!(my_atoi("42".to_string()), 42)
    }

    #[test]
    fn should_parse_negative_number() {
        assert_eq!(my_atoi("-42".to_string()), -42)
    }

    #[test]
    fn should_interpret_leading_plus_sign() {
        assert_eq!(my_atoi("+42".to_string()), 42)
    }

    #[test]
    fn should_parse_a_number_with_leading_whitespaces() {
        assert_eq!(my_atoi("       42".to_string()), 42)
    }

    #[test]
    fn should_parse_a_number_before_some_text() {
        assert_eq!(my_atoi("4193 with words".to_string()), 4193)
    }

    #[test]
    fn should_return_zero_if_non_numerics_are_in_front() {
        assert_eq!(my_atoi("words and 987".to_string()), 0)
    }

    #[test]
    fn should_return_min_int_if_negative_overflow_occurs() {
        assert_eq!(my_atoi("-91283472332".to_string()), i32::MIN)
    }

    #[test]
    fn should_return_max_int_if_positive_overflow_occurs() {
        assert_eq!(my_atoi("191283472332".to_string()), i32::MAX);
        assert_eq!(my_atoi("2147483648".to_string()), i32::MAX)
    }
}