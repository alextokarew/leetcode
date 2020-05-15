const BASE: i32 = 'A' as i32 - 1;

pub fn title_to_number(s: String) -> i32 {
    s.chars().fold(0, |acc, c| acc * 26 + (c as i32 - BASE))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_number() {
        assert_eq!(title_to_number("A".to_string()), 1);
        assert_eq!(title_to_number("Z".to_string()), 26);
        assert_eq!(title_to_number("AA".to_string()), 27);
        assert_eq!(title_to_number("AB".to_string()), 28);
        assert_eq!(title_to_number("ZY".to_string()), 701);
        assert_eq!(title_to_number("ABC".to_string()), 731);
    }

}