pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 1 {
        return 0;
    }
    
    let mut delta: usize = 0;
    let mut prev = nums[0];

    let mut i = 1;

    while i < nums.len() {
        if prev == nums[i] {
            delta += 1;
        } else if delta > 0 {
            nums[i - delta] = nums[i]; 
        }

        prev = nums[i];

        i += 1;
    }
    
    return (nums.len() - delta) as i32;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_length_if_no_duplicates() {
        let mut vec1 = vec![1,2,3,4,5];
        assert_eq!(remove_duplicates(&mut vec1), 5);
        assert_eq!(vec1, vec![1,2,3,4,5]);
        
        let mut vec2 = vec![1,3,5];
        assert_eq!(remove_duplicates(&mut vec2), 3);
        assert_eq!(vec2, vec![1,3,5]);
    }

    #[test]
    fn should_remove_duplicates_and_return_reduced_size() {
        let mut vec1 = vec![1,1,1,2,3,3,3,4,5,5];
        let result = remove_duplicates(&mut vec1);
        
        assert_eq!(result, 5);
        assert_eq!(vec1, vec![1,2,3,4,5,3,3,4,5,5]);
    }

    #[test]
    fn should_deal_with_zero_condition() {
        let mut vec0 = vec![];
        assert_eq!(remove_duplicates(&mut vec0), 0);
        assert_eq!(vec0, vec![]);
    }

    #[test]
    fn should_deal_with_single_element_vector() {
        let mut vec1 = vec![2];
        assert_eq!(remove_duplicates(&mut vec1), 1);
        assert_eq!(vec1, vec![2]);
    }
}