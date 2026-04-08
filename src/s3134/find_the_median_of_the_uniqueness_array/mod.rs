// Problem 3134: find the median of the uniqueness array
// #Hard #Array #Hash_Table #Binary_Search #Sliding_Window
// #2024_05_02_Time_47_ms_(100.00%)_Space_56.8_MB_(91.38%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let n = nums.len();
        let k = ((n as i64) * (n as i64 + 1) / 2 + 1) / 2;
        let mut left = 0;
        let mut right = n / 2;
        while left <= right {
            let mid = left + right >> 1;
            if Self::check(&nums, max, mid, k) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }

    fn check(nums: &[i32], _max: i32, target: usize, k: i64) -> bool {
        let mut count: i64 = 0;
        let mut distinct = 0;
        let n = nums.len();
        let mut left = 0;
        let mut right = 0;
        let mut freq: HashMap<i32, i32> = HashMap::new();
        while right < n {
            let x = nums[right];
            right += 1;
            let entry = freq.entry(x).or_insert(0);
            *entry += 1;
            if *entry == 1 {
                distinct += 1;
            }
            while distinct > target {
                let x = nums[left];
                left += 1;
                let entry = freq.get_mut(&x).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    distinct -= 1;
                }
            }
            count += (right - left) as i64;
            if count >= k {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_of_uniqueness_array() {
        assert_eq!(Solution::median_of_uniqueness_array(vec![1, 2, 3]), 1);
    }

    #[test]
    fn test_median_of_uniqueness_array2() {
        assert_eq!(Solution::median_of_uniqueness_array(vec![3, 4, 3, 4, 5]), 2);
    }

    #[test]
    fn test_median_of_uniqueness_array3() {
        assert_eq!(Solution::median_of_uniqueness_array(vec![4, 3, 5, 4]), 2);
    }
}
