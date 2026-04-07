// Problem 2281: sum of total strength of wizards

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn total_strength(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();

        // Find previous smaller element (strictly smaller)
        let mut left = vec
![n; n];
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..n {
            while !stack.is_empty() && nums[*stack.last().unwrap()] >= nums[i] {
                stack.pop();
            }
            left[i] = if stack.is_empty() { 0 } else { *stack.last().unwrap() + 1 };
            stack.push(i);
        }

        // Find next smaller or equal element
        stack.clear();
        let mut right = vec
![n; n];
        for i in (0..n).rev() {
            while !stack.is_empty() && nums[*stack.last().unwrap()] > nums[i] {
                stack.pop();
            }
            right[i] = if stack.is_empty() { n - 1 } else { *stack.last().unwrap() - 1 };
            stack.push(i);
        }

        // Prefix sum of prefix sum
        let mut pprefix = vec
![0i64; n + 2];
        let mut prefix = 0i64;
        for i in 1..=n {
            prefix = (prefix + nums[i - 1]) % MOD;
            pprefix[i + 1] = (pprefix[i] + prefix) % MOD;
        }

        let mut result = 0i64;
        for i in 0..n {
            let l = left[i];
            let r = right[i];
            // Number of subarrays starting at positions in [l, i] and ending at positions in [i, r]
            let cnt_l = (i - l + 1) as i64;
            let cnt_r = (r - i + 1) as i64;

            // Sum of subarray sums where nums[i] is minimum
            // Using inclusion-exclusion with prefix sums of prefix sums
            let sum_r = (pprefix[r + 2] - pprefix[i + 1] + MOD) % MOD;
            let sum_l = (pprefix[i + 1] - pprefix[l] + MOD) % MOD;
            let total = (sum_r * cnt_l % MOD - sum_l * cnt_r % MOD + MOD) % MOD;
            result = (result + nums[i] * total % MOD + MOD) % MOD;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void totalStrength()
    //   assertThat(new Solution().totalStrength(new int[] {1, 3, 1, 2}), equalTo(44));
    #[test]
    fn test_total_strength() {
        assert_eq!(Solution::total_strength(vec
![1, 3, 1, 2]), 44);
    }

    // Java: void totalStrength2()
    //   assertThat(new Solution().totalStrength(new int[] {5, 4, 6}), equalTo(213));
    #[test]
    fn test_total_strength2() {
        assert_eq!(Solution::total_strength(vec
![5, 4, 6]), 213);
    }
}
