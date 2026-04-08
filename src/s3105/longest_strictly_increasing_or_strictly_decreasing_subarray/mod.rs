// Problem 3105: longest strictly increasing or strictly decreasing subarray
// #Easy #Array #2024_04_11_Time_1_ms_(98.13%)_Space_42.7_MB_(57.07%)

pub struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut inc = 1;
        let mut dec = 1;
        let mut res = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                inc += 1;
                dec = 1;
            } else if nums[i] < nums[i - 1] {
                dec += 1;
                inc = 1;
            } else {
                inc = 1;
                dec = 1;
            }
            res = res.max(inc.max(dec));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_monotonic_subarray() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    }

    #[test]
    fn test_longest_monotonic_subarray2() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    }
}
