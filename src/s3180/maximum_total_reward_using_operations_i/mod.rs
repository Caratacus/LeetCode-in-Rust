// Problem 3180: maximum total reward using operations i
// #Medium #Array #Dynamic_Programming #2024_06_14_Time_1_ms_(100.00%)_Space_43.3_MB_(97.85%)

pub struct Solution;

impl Solution {
    fn sorted_set(values: &[i32]) -> Vec<i32> {
        let max = *values.iter().max().unwrap_or(&0);
        let mut set = vec![false; (max + 1) as usize];
        let mut n = 0;
        for &x in values {
            if !set[x as usize] {
                set[x as usize] = true;
                n += 1;
            }
        }
        let mut result = vec![0; n];
        for x in (1..=max).rev() {
            if set[x as usize] {
                n -= 1;
                result[n] = x;
            }
        }
        result
    }

    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let reward_values = Self::sorted_set(&reward_values);
        let n = reward_values.len();
        let max = reward_values[n - 1];
        let mut is_sum_possible = vec![false; max as usize];
        is_sum_possible[0] = true;
        let mut max_sum = 0;
        let mut last = 1;
        for sum in reward_values[0]..max {
            while last < n && reward_values[last] <= sum {
                last += 1;
            }
            let s2 = sum / 2;
            for i in (0..last).rev() {
                let x = reward_values[i];
                if x <= s2 {
                    break;
                }
                if is_sum_possible[(sum - x) as usize] {
                    is_sum_possible[sum as usize] = true;
                    max_sum = sum;
                    break;
                }
            }
        }
        max_sum + max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxTotalReward()
    //   assertThat(new Solution().maxTotalReward(new int[] {1, 1, 3, 3}), equalTo(4));
    #[test]
    fn test_max_total_reward() {
        assert_eq!(Solution::max_total_reward(vec![1, 1, 3, 3]), 4);
    }

    // Java: void maxTotalReward2()
    //   assertThat(new Solution().maxTotalReward(new int[] {1, 6, 4, 3, 2}), equalTo(11));
    #[test]
    fn test_max_total_reward2() {
        assert_eq!(Solution::max_total_reward(vec![1, 6, 4, 3, 2]), 11);
    }
}
