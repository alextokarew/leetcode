pub fn reverse(x: i32) -> i32 {
    let mut out: i64 = 0;
    let mut acc = x as i64;

    while acc != 0 {
        out = out * 10 + acc % 10;
        acc = acc / 10
    }

    if out >= std::i32::MIN.into() && out <= std::i32::MAX.into() {
        out as i32
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_should_work() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn reverse_should_deal_with_negative_numbers() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn reverse_should_deal_with_overflow() {
        assert_eq!(reverse(2147483641), 1463847412);
        assert_eq!(reverse(1534236469), 0);
        assert_eq!(reverse(2147483647), 0)
    }

}