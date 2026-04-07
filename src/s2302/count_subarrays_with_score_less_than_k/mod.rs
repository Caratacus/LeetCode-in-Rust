// Problem 2302: count subarrays with score less than k

pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();
        let mut count = 0i64;
        let mut sum = 0i64;
        let mut left = 0usize;
        for right in 0..n {
            sum += nums[right] as i64;
            while left <= right && sum * (right - left + 1) as i64 >= k {
                sum -= nums[left] as i64;
                left += 1;
            }
            count += (right - left + 1) as i64;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countSubarrays()
    //   assertThat(new Solution().countSubarrays(new int[] {2, 1, 4, 3, 5}, 10), equalTo(6L));
    #[test]
    fn test_count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec
![2, 1, 4, 3, 5], 10), 6);
    }

    // Java: void countSubarrays2()
    //   assertThat(new Solution().countSubarrays(new int[] {1, 1, 1}, 5), equalTo(5L));
    #[test]
    fn test_count_subarrays2() {
        assert_eq!(Solution::count_subarrays(vec
![1, 1, 1], 5), 5);
    }
}
