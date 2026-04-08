// Problem 3149: find the minimum cost array permutation
// #Hard #Array #Dynamic_Programming #Bit_Manipulation #Bitmask
// #2024_05_15_Time_105_ms_(88.11%)_Space_46.5_MB_(64.32%)

pub struct Solution;

impl Solution {
    fn find_min_score(mask: usize, prev_num: usize, nums: &[i32], dp: &mut Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        if mask.count_ones() as usize == n {
            dp[mask][prev_num] = (prev_num as i32 - nums[0] as i32).abs();
            return dp[mask][prev_num];
        }
        if dp[mask][prev_num] != -1 {
            return dp[mask][prev_num];
        }
        let mut min_score = i32::MAX;
        for curr_num in 0..n {
            if ((mask >> curr_num) & 1) ^ 1 == 1 {
                let curr_score = (prev_num as i32 - nums[curr_num] as i32).abs()
                    + Self::find_min_score(mask | (1 << curr_num), curr_num, nums, dp);
                min_score = min_score.min(curr_score);
            }
        }
        dp[mask][prev_num] = min_score;
        min_score
    }

    fn construct_min_score_permutation(n: usize, nums: &[i32], dp: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut permutation = vec![0; n];
        let mut i = 1;
        permutation[0] = 0;
        let mut prev_num = 0;
        let mut mask = 1;

        while i < n {
            for curr_num in 0..n {
                if ((mask >> curr_num) & 1) ^ 1 == 1 {
                    let curr_score =
                        (prev_num as i32 - nums[curr_num] as i32).abs() + dp[mask | (1 << curr_num)][curr_num];
                    let min_score = dp[mask][prev_num];
                    if curr_score == min_score {
                        permutation[i] = curr_num as i32;
                        i += 1;
                        prev_num = curr_num;
                        mask |= 1 << prev_num;
                        break;
                    }
                }
            }
        }
        permutation
    }

    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut dp = vec![vec![-1; n]; 1 << n];
        Self::find_min_score(1, 0, &nums, &mut dp);
        Self::construct_min_score_permutation(n, &nums, &dp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_permutation() {
        assert_eq!(
            Solution::find_permutation(vec![1, 0, 2]),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn test_find_permutation2() {
        assert_eq!(
            Solution::find_permutation(vec![0, 2, 1]),
            vec![0, 2, 1]
        );
    }
}
