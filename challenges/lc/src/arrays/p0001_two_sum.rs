use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;

        if complements.contains_key(&complement) {
            return vec![complements[&complement], i as i32];
        }

        complements.insert(*num, i as i32);
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let matches = vec![0, 1];

        assert_eq!(two_sum(nums, target), matches);
    }

    #[test]
    fn test_two_sum_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let matches = vec![1, 2];

        assert_eq!(two_sum(nums, target), matches);
    }

    #[test]
    fn test_two_sum_3() {
        let nums = vec![3, 3];
        let target = 6;

        let matches = vec![0, 1];

        assert_eq!(two_sum(nums, target), matches);
    }
}
