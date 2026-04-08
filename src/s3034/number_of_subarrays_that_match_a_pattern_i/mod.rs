// Problem 3034: number of subarrays that match a pattern i
// #Medium #Array #Hash_Function #String_Matching #Rolling_Hash
// #2024_03_01_Time_1_ms_(100.00%)_Space_43.9_MB_(97.20%)

pub struct Solution;

impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = pattern.len();
        let mut count = 0;
        for i in 0..=n - m - 1 {
            let mut k = 0;
            while k < m {
                if nums[i + k + 1] > nums[i + k] && pattern[k] == 1 {
                    k += 1;
                } else if nums[i + k + 1] == nums[i + k] && pattern[k] == 0 {
                    k += 1;
                } else if nums[i + k + 1] < nums[i + k] && pattern[k] == -1 {
                    k += 1;
                } else {
                    break;
                }
            }
            if k == m {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matching_subarrays() {
        assert_eq!(Solution::count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]), 4);
    }

    #[test]
    fn test_count_matching_subarrays2() {
        assert_eq!(
            Solution::count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]),
            2
        );
    }
}
