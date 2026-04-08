// Problem 3101: Count Alternating Subarrays
// #Medium #Array #Math #2024_04_20_Time_3_ms_(97.51%)_Space_56.4_MB_(31.27%)

pub struct Solution;

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut count: i64 = 0;
        let mut start = 0;
        let mut end = 1;
        while end < nums.len() {
            if nums[end] != nums[end - 1] {
                end += 1;
            } else {
                let length = end as i64 - start as i64;
                count += (length * (length + 1)) / 2;
                start = end;
                end += 1;
            }
        }
        let length = end as i64 - start as i64;
        count += (length * (length + 1)) / 2;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countAlternatingSubarrays()
    //   assertThat(new Solution().countAlternatingSubarrays(new int[] {0, 1, 1, 1}), equalTo(5L));
    #[test]
    fn test_count_alternating_subarrays() {
        assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
    }

    // Java: void countAlternatingSubarrays2()
    //   assertThat(new Solution().countAlternatingSubarrays(new int[] {1, 0, 1, 0}), equalTo(10L));
    #[test]
    fn test_count_alternating_subarrays2() {
        assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10);
    }
}
