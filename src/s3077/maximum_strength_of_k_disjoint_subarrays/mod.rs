// Problem 3077: maximum strength of k disjoint subarrays
// #Hard #Array #Dynamic_Programming #Prefix_Sum
// #2024_04_16_Time_20_ms_(97.16%)_Space_56.3_MB_(71.00%)

pub struct Solution;

impl Solution {
    pub fn maximum_strength(n: Vec<i32>, k: i32) -> i64 {
        if n.len() == 1 {
            return n[0] as i64;
        }

        let k = k as usize;
        let len = n.len();
        let mut dp = vec![vec![0i64; k]; len];

        dp[0][0] = (k as i64) * (n[0] as i64);

        for i in 1..k {
            let mut pm: i64 = -1;
            dp[i][0] = std::cmp::max(0, dp[i - 1][0]) + (k as i64) * (n[i] as i64);
            for j in 1..i {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i - 1][j - 1])
                    + ((k - j) as i64) * (n[i] as i64) * pm;
                pm = -pm;
            }
            dp[i][i] = dp[i - 1][i - 1] + ((k - i) as i64) * (n[i] as i64) * pm;
        }

        let mut max = dp[k - 1][k - 1];

        for i in k..len {
            let mut pm: i64 = 1;
            dp[i][0] = std::cmp::max(0, dp[i - 1][0]) + (k as i64) * (n[i] as i64);
            for j in 1..k {
                pm = -pm;
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i - 1][j - 1])
                    + ((k - j) as i64) * (n[i] as i64) * pm;
            }
            if max < dp[i][k - 1] {
                max = dp[i][k - 1];
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumStrength()
    //   assertThat(new Solution().maximumStrength(new int[] {1, 2, 3, -1, 2}, 3), equalTo(22L));
    #[test]
    fn test_maximum_strength() {
        assert_eq!(Solution::maximum_strength(vec![1, 2, 3, -1, 2], 3), 22);
    }

    // Java: void maximumStrength2()
    //   assertThat(new Solution().maximumStrength(new int[] {12, -2, -2, -2, -2}, 5), equalTo(64L));
    #[test]
    fn test_maximum_strength2() {
        assert_eq!(Solution::maximum_strength(vec![12, -2, -2, -2, -2], 5), 64);
    }

    // Java: void maximumStrength3()
    //   assertThat(new Solution().maximumStrength(new int[] {-1, -2, -3}, 1), equalTo(-1L));
    #[test]
    fn test_maximum_strength3() {
        assert_eq!(Solution::maximum_strength(vec![-1, -2, -3], 1), -1);
    }
}
