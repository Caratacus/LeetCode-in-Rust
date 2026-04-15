// Problem 3381: maximum subarray sum with length divisible by k

pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut max_sum = vec![0i64; n];
        let mut min_sum: i64 = 0;
        for i in (n - k + 1..n).rev() {
            max_sum[i] = i32::MIN as i64;
            min_sum += nums[i] as i64;
        }
        min_sum += nums[n - k] as i64;
        max_sum[n - k] = min_sum;
        let mut ans = min_sum;
        for i in (0..n - k).rev() {
            min_sum = min_sum + nums[i] as i64 - nums[i + k] as i64;
            max_sum[i] = min_sum.max(min_sum.wrapping_add(max_sum[i + k]));
            ans = ans.max(max_sum[i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxSubarraySum()
    //   assertThat(new Solution().maxSubarraySum(new int[] {1, 2}, 1), equalTo(3L));
    #[test]
    fn test_max_subarray_sum() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxSubarraySum2()
    //   assertThat(new Solution().maxSubarraySum(new int[] {-1, -2, -3, -4, -5}, 4), equalTo(-10L));
    #[test]
    fn test_max_subarray_sum2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxSubarraySum3()
    //   assertThat(new Solution().maxSubarraySum(new int[] {-5, 1, 2, -3, 4}, 2), equalTo(4L));
    #[test]
    fn test_max_subarray_sum3() {
        // TODO: 翻译 Java 测试
    }
}
