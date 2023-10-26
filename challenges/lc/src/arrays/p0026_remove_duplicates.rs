fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut idx = 1;
    let mut ins_index = 1;

    while idx < nums.len() {
        if nums[idx] != nums[idx - 1] {
            nums[ins_index] = nums[idx];
            ins_index += 1;
        }
        idx += 1;
    }

    nums.truncate(ins_index);
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_001() {
        let mut nums = vec![1, 1, 2, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums), 3);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_duplicates_002() {
        let mut nums = vec![1, 1, 1, 1, 1, 1];
        assert_eq!(remove_duplicates(&mut nums), 1);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_remove_duplicates_003() {
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(remove_duplicates(&mut nums), 6);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_remove_duplicates_004() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}
