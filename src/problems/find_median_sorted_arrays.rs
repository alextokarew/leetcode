use std::cmp::{min, max};

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.is_empty() && nums2.is_empty() {
        return 0.0
    }

    let take_middle = (nums1.len() + nums2.len()) / 2;

    let (little, big) = if nums1.len() < nums2.len() { (nums1, nums2) } else { (nums2, nums1) };
    let take_little_max = min(take_middle, little.len());
    let mut from = 0;
    let mut to = take_little_max + 1;

    let mut take_little = 0;
    let mut take_big = take_middle - take_little;

    while take_little < take_little_max && ( 
        (take_little > 0 && little[take_little - 1] > big[take_big]) || 
        (take_big > 0 && big[take_big - 1] > little[take_little])) {
 
        if take_little > 0 && little[take_little - 1] > big[take_big] {
            to = take_little
        } else {
            from = take_little
        }

        take_little = (from + to) / 2;
        take_big = take_middle - take_little;
    }

    if (little.len() + big.len()) % 2 == 1 {
        (if take_little == little.len() { big[take_big] } else { min(little[take_little], big[take_big]) }) as f64
    } else {
        let first = (if take_little == 0 { big[take_big - 1] } else if take_big == 0 { little[take_little - 1] } else { max(little[take_little - 1], big[take_big - 1]) }) as f64;
        let second = (if take_little == little.len() { big[take_big] } else if take_big == big.len() { little[take_little] } else { min(little[take_little], big[take_big]) }) as f64;
        (first + second) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_for_equal_sized_vectors() {
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5)
    }

    #[test]
    fn should_calculate_for_odd_total_length() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![6, 8, 10, 11]), 7.0) 
    }

    #[test]
    fn should_calculate_for_even_total_length() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![6, 8, 10]), 6.5)
    }

    #[test]
    fn should_calculate_when_lengths_are_very_different() {
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![9, 10, 11]), 6.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6, 7, 8, 9, 10, 11]), 6.0)
    }

    #[test]
    fn should_calculate_when_lengths_are_even_and_different() {
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![10, 11, 12]), 6.5);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12]), 6.5)
    }

    #[test]
    fn should_calculate_for_small_vectors() {
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3]), 2.0) 
    }

    #[test]
    fn should_work_if_one_of_vectors_is_empty() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![]), 5.0);
        assert_eq!(find_median_sorted_arrays(vec![3, 5], vec![]), 4.0);
        assert_eq!(find_median_sorted_arrays(vec![3], vec![]), 3.0);
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }

    #[test]
    fn should_return_zero_if_both_are_empty() {
        assert_eq!(find_median_sorted_arrays(vec![], vec![]), 0.0)
    }
}