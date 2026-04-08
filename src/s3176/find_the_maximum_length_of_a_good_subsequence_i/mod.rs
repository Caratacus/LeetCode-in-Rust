// Problem 3176: find the maximum length of a good subsequence i
// #Medium #Array #Hash_Table #Dynamic_Programming
// #2024_06_12_Time_4_ms_(99.70%)_Space_44_MB_(87.51%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut count = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] != nums[i + 1] {
                count += 1;
            }
        }
        if count <= k {
            return n as i32;
        }
        let k = k as usize;
        let mut max = vec![1i32; k + 1];
        let mut vis = vec![-1i32; n];
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..n {
            if !map.contains_key(&nums[i]) {
                map.insert(nums[i], (i + 1) as i32);
            } else {
                vis[i] = map.get(&nums[i]).unwrap() - 1;
                map.insert(nums[i], (i + 1) as i32);
            }
        }
        let mut dp = vec![vec![1i32; k + 1]; n];
        for i in 1..n {
            for j in (0..k).rev() {
                dp[i][j + 1] = dp[i][j + 1].max(1 + max[j]);
                max[j + 1] = max[j + 1].max(dp[i][j + 1]);
            }
            if vis[i] != -1 {
                let a = vis[i] as usize;
                for j in 0..=k {
                    dp[i][j] = dp[i][j].max(1 + dp[a][j]);
                    max[j] = max[j].max(dp[i][j]);
                }
            }
        }
        let mut ans = 1;
        for i in 0..=k {
            ans = ans.max(max[i]);
        }
        ans
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
