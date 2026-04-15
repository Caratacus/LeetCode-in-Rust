// Problem 3392: count subarrays of length three with a condition

pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        for i in 0..=nums.len() - 3 {
            if nums[i + 1] % 2 == 0 && nums[i + 1] / 2 == nums[i] + nums[i + 2] {
                cnt += 1;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countSubarrays()
    //   assertThat(new Solution().countSubarrays(new int[] {1, 2, 1, 4, 1}), equalTo(1));
    #[test]
    fn test_count_subarrays() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countSubarrays2()
    //   assertThat(new Solution().countSubarrays(new int[] {1, 1, 1}), equalTo(0));
    #[test]
    fn test_count_subarrays2() {
        // TODO: 翻译 Java 测试
    }
}
