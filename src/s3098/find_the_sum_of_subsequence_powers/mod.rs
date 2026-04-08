// Problem 3098: find the sum of subsequence powers
// #Hard #Array #Dynamic_Programming #Sorting #2024_04_19_Time_34_ms_(91.54%)_Space_47.9_MB_(65.64%)

use std::collections::HashMap;

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn dfs(
        last_idx: usize,
        k: i32,
        min_diff: i32,
        dp: &mut HashMap<(usize, i32, i32), i32>,
        nums: &[i32],
        len: usize,
    ) -> i32 {
        if k == 0 {
            return min_diff;
        }
        let key = (last_idx, k, min_diff);
        if let Some(&val) = dp.get(&key) {
            return val;
        }

        let mut res: i64 = 0;
        for i in last_idx + 1..=len - k as usize {
            res = (res
                + Self::dfs(
                    i,
                    k - 1,
                    std::cmp::min(min_diff, nums[i] - nums[last_idx]),
                    dp,
                    nums,
                    len,
                ) as i64)
                % MOD;
        }
        let result = res as i32;
        dp.insert(key, result);
        result
    }

    pub fn sum_of_powers(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        nums.sort();

        let mut dp: HashMap<(usize, i32, i32), i32> = HashMap::new();
        let mut res: i64 = 0;

        for i in 0..=len - k as usize {
            res = (res
                + Self::dfs(
                    i,
                    k - 1,
                    nums[len - 1] - nums[0],
                    &mut dp,
                    &nums,
                    len,
                ) as i64)
                % MOD;
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sumOfPowers()
    //   assertThat(new Solution().sumOfPowers(new int[] {1, 2, 3, 4}, 3), equalTo(4));
    #[test]
    fn test_sum_of_powers() {
        assert_eq!(Solution::sum_of_powers(vec![1, 2, 3, 4], 3), 4);
    }

    // Java: void sumOfPowers2()
    //   assertThat(new Solution().sumOfPowers(new int[] {2, 2}, 2), equalTo(0));
    #[test]
    fn test_sum_of_powers2() {
        assert_eq!(Solution::sum_of_powers(vec![2, 2], 2), 0);
    }

    // Java: void sumOfPowers3()
    //   assertThat(new Solution().sumOfPowers(new int[] {4, 3, -1}, 2), equalTo(10));
    #[test]
    fn test_sum_of_powers3() {
        assert_eq!(Solution::sum_of_powers(vec![4, 3, -1], 2), 10);
    }
}
