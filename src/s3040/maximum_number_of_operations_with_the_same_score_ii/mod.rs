// Problem 3040: maximum number of operations with the same score ii
// #Medium #Array #Dynamic_Programming #Memoization
// #2024_03_04_Time_3_ms_(99.75%)_Space_44.6_MB_(99.29%)

use std::collections::HashMap;
use std::mem;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }

        let mut dp: HashMap<(usize, usize, i32), i32> = HashMap::new();
        let mut max_ops = 1;

        Self::max_operations_helper(
            &nums,
            2,
            n - 1,
            nums[0] + nums[1],
            1,
            &mut max_ops,
            &mut dp
        );
        if n >= 3 {
            Self::max_operations_helper(
                &nums,
                0,
                n - 3,
                nums[n - 2] + nums[n - 1],
                1,
                &mut max_ops,
                &mut dp
            );
        }
        Self::max_operations_helper(
            &nums,
            1,
            n - 2,
            nums[0] + nums[n - 1],
            1,
            &mut max_ops,
            &mut dp
        );

        max_ops
    }

    fn max_operations_helper(
        nums: &[i32],
        start: usize,
        end: usize,
        sum: i32,
        n_ops: i32,
        max_ops: &mut i32,
        dp: &mut HashMap<(usize, usize, i32), i32>,
    ) {
        if start >= end {
            return;
        }

        let remaining = ((end - start) / 2) as i32 + n_ops;
        if remaining < *max_ops {
            return;
        }

        let pos = (start, end, sum);
        if let Some(&pos_nops) = dp.get(&pos) {
            if pos_nops >= n_ops {
                return;
            }
        }
        dp.insert(pos, n_ops);

        // nums[start] + nums[start + 1] == sum
        if start + 1 <= end && nums[start] + nums[start + 1] == sum {
            *max_ops = (*max_ops).max(n_ops + 1);
            Self::max_operations_helper(nums, start + 2, end, sum, n_ops + 1, max_ops, dp);
        }

        // nums[end - 1] + nums[end] == sum
        if end >= 1 && nums[end - 1] + nums[end] == sum {
            *max_ops = (*max_ops).max(n_ops + 1);
            if end >= 2 {
                Self::max_operations_helper(nums, start, end - 2, sum, n_ops + 1, max_ops, dp);
            }
        }

        // nums[start] + nums[end] == sum
        if nums[start] + nums[end] == sum {
            *max_ops = (*max_ops).max(n_ops + 1);
            if end >= 1 {
                Self::max_operations_helper(nums, start + 1, end - 1, sum, n_ops + 1, max_ops, dp);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_operations() {
        let nums = vec![3, 2, 1, 2, 3, 4];
        assert_eq!(Solution::max_operations(nums), 3);
    }

    #[test]
    fn test_max_operations2() {
        let nums = vec![3, 2, 6, 1, 4];
        assert_eq!(Solution::max_operations(nums), 2);
    }

    #[test]
    fn test_max_operations3() {
        let nums = vec![1, 2];
        assert_eq!(Solution::max_operations(nums), 1);
    }

    #[test]
    fn test_max_operations4() {
        let nums = vec![1, 1, 1];
        assert_eq!(Solution::max_operations(nums), 1);
    }

    #[test]
    fn test_max_operations5() {
        let nums = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
        assert_eq!(Solution::max_operations(nums), 3);
    }

    #[test]
    fn test_max_operations6() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::max_operations(nums), 3);
    }

    #[test]
    fn test_max_operations7() {
        let nums = vec![6, 5, 4, 3, 2, 1];
        assert_eq!(Solution::max_operations(nums), 3);
    }

    #[test]
    fn test_max_operations8() {
        let nums = vec![1, 3, 2, 4, 1, 3];
        assert_eq!(Solution::max_operations(nums), 2);
    }

    #[test]
    fn test_max_operations9() {
        let nums = vec![1, 2, 4, 5];
        assert_eq!(Solution::max_operations(nums), 2);
    }

    #[test]
    fn test_max_operations10() {
        let nums = vec![5, 5];
        assert_eq!(Solution::max_operations(nums), 1);
    }
}
