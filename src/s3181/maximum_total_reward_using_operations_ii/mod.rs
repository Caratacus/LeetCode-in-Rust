// Problem 3181: maximum total reward using operations ii
// #Hard #Array #Dynamic_Programming #Bit_Manipulation
// #2024_06_14_Time_2_ms_(100.00%)_Space_53.3_MB_(90.35%)

pub struct Solution;

impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let max = *reward_values.iter().max().unwrap_or(&0);
        let mut vis = vec![false; (max + 1) as usize];
        let mut n = 0;
        for &i in &reward_values {
            if !vis[i as usize] {
                n += 1;
                vis[i as usize] = true;
            }
        }
        let mut rew = vec![0; n];
        let mut j = 0;
        for i in 0..=max {
            if vis[i as usize] {
                rew[j] = i;
                j += 1;
            }
        }
        rew[n - 1] + Self::get_ans(&rew, n - 1, rew[n - 1] - 1)
    }

    fn get_ans(rewards: &[i32], i: usize, valid_limit: i32) -> i32 {
        let mut res = 0;
        let mut j = Self::next_elem_within_limits(rewards, i as i32 - 1, valid_limit);
        while j >= 0 {
            if res >= rewards[j as usize] + std::cmp::min(valid_limit - rewards[j as usize], rewards[j as usize] - 1) {
                break;
            }
            res = res.max(
                rewards[j as usize]
                    + Self::get_ans(
                        rewards,
                        j as usize,
                        std::cmp::min(valid_limit - rewards[j as usize], rewards[j as usize] - 1),
                    ),
            );
            j -= 1;
        }
        res
    }

    fn next_elem_within_limits(rewards: &[i32], h: i32, k: i32) -> i32 {
        let mut l = 0;
        let mut h = h;
        let mut res_ind = -1;
        while l <= h {
            let m = (l + h) / 2;
            if rewards[m as usize] <= k {
                res_ind = m;
                l = m + 1;
            } else {
                h = m - 1;
            }
        }
        res_ind
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
