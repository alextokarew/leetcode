use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut buffer: HashMap<i32, i32> = HashMap::new();
    for (uindex, num) in nums.iter().enumerate() {
        let index = uindex as i32;
        match buffer.get(&num) {
            Some(&other_index) => return vec![other_index, index],
            None => buffer.insert(target - num, index)
        };
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_first_pair() {
        assert_eq!(two_sum(vec![2, 7, 11, 19], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![2, 7, 11, 19], 18), vec![1, 2]);
    }

    #[test]
    fn should_return_empty_vec_when_not_found() {
        assert_eq!(two_sum(vec![2, 7, 11, 19], 10), vec![]);
    }
}