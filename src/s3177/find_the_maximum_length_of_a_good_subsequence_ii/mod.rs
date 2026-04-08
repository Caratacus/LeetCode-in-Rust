// Problem 3177: find the maximum length of a good subsequence ii
// #Hard #Array #Hash_Table #Dynamic_Programming
// #2024_06_12_Time_11_ms_(100.00%)_Space_45.8_MB_(90.55%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let n = nums.len();
        let k = k as usize;
        let mut pre = vec![-1i32; n];
        for i in 0..n {
            pre[i] = *hm.get(&nums[i]).unwrap_or(&-1);
            hm.insert(nums[i], i as i32);
        }
        let mut dp = vec![vec![0i32; n]; k + 1];
        for i in 0..n {
            dp[0][i] = 1;
            if pre[i] >= 0 {
                dp[0][i] = dp[0][pre[i] as usize] + 1;
            }
        }
        for i in 1..=k {
            let mut max = 0;
            for j in 0..n {
                if pre[j] >= 0 {
                    dp[i][j] = dp[i][pre[j] as usize] + 1;
                }
                dp[i][j] = dp[i][j].max(max + 1);
                max = max.max(dp[i - 1][j]);
            }
        }
        let mut max = 0;
        for i in 0..n {
            max = max.max(dp[k][i]);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumLength()
    //   assertThat(new Solution().maximumLength(new int[] {1, 2, 1, 1, 3}, 2), equalTo(4));
    #[test]
    fn test_maximum_length() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
    }

    // Java: void maximumLength2()
    //   assertThat(new Solution().maximumLength(new int[] {1, 2, 3, 4, 5, 1}, 0), equalTo(2));
    #[test]
    fn test_maximum_length2() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
    }
}
