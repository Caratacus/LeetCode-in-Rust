// Problem 3036: number of subarrays that match a pattern ii
// #Hard #Array #Hash_Function #String_Matching #Rolling_Hash
// #2024_03_01_Time_5_ms_(98.27%)_Space_172.1_MB_(75.77%)

pub struct Solution;

impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = pattern.len();
        let mut arr = vec![0; n - 1];

        for i in 0..n - 1 {
            if nums[i + 1] > nums[i] {
                arr[i] = 1;
            } else if nums[i + 1] < nums[i] {
                arr[i] = -1;
            }
        }

        let mut hash: i64 = 0;
        let mut p_hash: i64 = 0;
        let mut base: i64 = 1;

        for i in 0..m {
            hash = hash * 3 + (arr[i] + 1) as i64;
            p_hash = p_hash * 3 + (pattern[i] + 1) as i64;
            base *= 3;
        }

        let mut count = 0;
        for i in 0..=n - 1 - m {
            if hash == p_hash {
                count += 1;
            }

            if i < n - 1 - m {
                hash = hash * 3 - base * (arr[i] + 1) as i64 + (arr[i + m] + 1) as i64;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countMatchingSubarrays()
    //   assertThat(
    //   new Solution()
    //   .countMatchingSubarrays(new int[] {1, 2, 3, 4, 5, 6}, new int[] {1, 1}),
    //   equalTo(4));
    #[test]
    fn test_count_matching_subarrays() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let pattern = vec![1, 1];
        assert_eq!(Solution::count_matching_subarrays(nums, pattern), 4);
    }

    // Java: void countMatchingSubarrays2()
    //   assertThat(
    //   new Solution()
    //   .countMatchingSubarrays(
    //   new int[] {1, 4, 4, 1, 3, 5, 5, 3}, new int[] {1, 0, -1}),
    //   equalTo(2));
    #[test]
    fn test_count_matching_subarrays2() {
        let nums = vec![1, 4, 4, 1, 3, 5, 5, 3];
        let pattern = vec![1, 0, -1];
        assert_eq!(Solution::count_matching_subarrays(nums, pattern), 2);
    }
}
