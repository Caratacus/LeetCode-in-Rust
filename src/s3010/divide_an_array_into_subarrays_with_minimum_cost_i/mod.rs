// Problem 3010: divide an array into subarrays with minimum cost i
// #Easy #Array #Sorting #Enumeration #2024_02_27_Time_1_ms_(99.09%)_Space_43.6_MB_(96.36%)

pub struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        let mut min = 51;
        let mut sec_min = 52;
        for i in 1..nums.len() {
            if nums[i] < min {
                sec_min = min;
                min = nums[i];
            } else if nums[i] < sec_min {
                sec_min = nums[i];
            }
        }
        first + min + sec_min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_cost() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 12]), 6);
    }

    #[test]
    fn test_minimum_cost2() {
        assert_eq!(Solution::minimum_cost(vec![5, 4, 3]), 12);
    }

    #[test]
    fn test_minimum_cost3() {
        assert_eq!(Solution::minimum_cost(vec![10, 3, 1, 1]), 12);
    }
}
