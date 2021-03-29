pub fn convert(s: String, num_rows: i32) -> String {
    if s.is_empty() || num_rows == 1 {
        return s;
    }

    let num_rows = num_rows as usize;

    let s_bytes = s.as_bytes();
    let mut result = Vec::from(s_bytes);

    let period = 2*(num_rows - 1) as usize;
    let mut row: usize = 0;
    let mut s_position = 0;
    let mut reminder = 0;

    for i in 1..s.len() {
        s_position += if row == 0 || row == num_rows - 1 {
            period
        } else {
            if reminder > 0 {
                let delta = reminder;
                reminder = 0;
                delta
            } else {
                let delta = period - row*2;
                reminder = period - delta;
                delta
            }
        };

        if s_position >= s.len() {
            row += 1;
            s_position = row;
            reminder = 0;
        }

        result[i] = s_bytes[s_position];
    }

    return String::from_utf8(result).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_perform_the_conversion() {
        assert_eq!(convert(String::from("PAYPALISHIRING"), 3), String::from("PAHNAPLSIIGYIR"));
        assert_eq!(convert(String::from("PAYPALISHIRING"), 4), String::from("PINALSIGYAHRPI"));
    }

    #[test]
    fn should_return_original_string_when_num_rows_is_1() {
        assert_eq!(convert(String::from("PAYPALISHIRING"), 1), String::from("PAYPALISHIRING"));
    }

    #[test]
    fn should_deal_with_empty_string() {
        assert_eq!(convert(String::from(""), 3), String::from(""));
        assert_eq!(convert(String::from(""), 4), String::from(""));
    }
}