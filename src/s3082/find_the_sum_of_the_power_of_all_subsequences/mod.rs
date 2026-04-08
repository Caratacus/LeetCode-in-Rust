// Problem 3082: find the sum of the power of all subsequences
// #Hard #Array #Dynamic_Programming

pub struct Solution;

impl Solution {
    pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
        const K_MOD: i64 = 1_000_000_007;
        let k = k as usize;
        let mut dp = vec![0i64; k + 1];
        dp[0] = 1;

        for &num in &nums {
            let num = num as usize;
            for i in (0..=k).rev() {
                if i < num {
                    dp[i] = (dp[i] * 2) % K_MOD;
                } else {
                    dp[i] = (dp[i] * 2 + dp[i - num]) % K_MOD;
                }
            }
        }

        dp[k] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sumOfPower()
    //   assertThat(new Solution().sumOfPower(new int[] {2, 3, 3}, 5), equalTo(4));
    #[test]
    fn test_sum_of_power() {
        assert_eq!(Solution::sum_of_power(vec![2, 3, 3], 5), 4);
    }

    // Java: void sumOfPower2()
    //   assertThat(new Solution().sumOfPower(new int[] {1, 2, 3}, 7), equalTo(0));
    #[test]
    fn test_sum_of_power2() {
        assert_eq!(Solution::sum_of_power(vec![1, 2, 3], 7), 0);
    }
}
