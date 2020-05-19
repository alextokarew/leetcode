pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut reversed = 0;
    let mut acc = x;

    while acc != 0 {
        reversed = reversed * 10 + acc % 10;
        acc = acc / 10
    }

    x == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_for_palindrome() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(1221), true);
        assert_eq!(is_palindrome(1234321), true);
    }

    #[test]
    fn should_return_false_for_negative_numbers() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn should_return_true_for_single_digits() {
        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(1), true);
    }

}