// Problem 3117: Minimum Sum of Values by Dividing Array
// #Hard #Array #Dynamic_Programming #Binary_Search #Bit_Manipulation #Queue #Segment_Tree
// #2024_04_27_Time_6_ms_(100.00%)_Space_44.8_MB_(99.04%)

const INF: i32 = 0xfffffff;

pub struct Solution;

impl Solution {
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![INF; n + 1];
        dp[0] = 0;
        for target in &and_values {
            let mut sum = INF;
            let mut min_sum = INF;
            let mut right_sum = INF;
            let mut left_sum = vec![INF; n + 1];
            left_sum[0] = INF;
            let mut left = 0;
            let mut right = 0;
            let mut nextdp = vec![INF; n + 1];
            nextdp[0] = INF;
            for i in 0..n {
                sum &= nums[i];
                right_sum &= nums[i];
                right += 1;
                if sum < *target {
                    min_sum = INF;
                    sum = nums[i];
                }
                while (left_sum[left] & right_sum) <= *target {
                    if (left_sum[left] & right_sum) == *target {
                        min_sum = min_sum.min(dp[i - left - right + 1]);
                    }
                    if left > 0 {
                        left -= 1;
                    } else {
                        left = right;
                        let mut start = i;
                        for l in 1..=left {
                            left_sum[l] = left_sum[l - 1] & nums[start];
                            start -= 1;
                        }
                        right = 0;
                        right_sum = INF;
                    }
                }
                nextdp[i + 1] = min_sum + nums[i];
            }
            dp = nextdp;
        }
        if dp[n] < INF { dp[n] } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumValueSum()
    //   assertThat(
    //   new Solution().minimumValueSum(new int[] {1, 4, 3, 3, 2}, new int[] {0, 3, 3, 2}),
    //   equalTo(12));
    #[test]
    fn test_minimum_value_sum() {
        assert_eq!(
            Solution::minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2]),
            12
        );
    }

    // Java: void minimumValueSum2()
    //   assertThat(
    //   new Solution()
    //   .minimumValueSum(new int[] {2, 3, 5, 7, 7, 7, 5}, new int[] {0, 7, 5}),
    //   equalTo(17));
    #[test]
    fn test_minimum_value_sum2() {
        assert_eq!(
            Solution::minimum_value_sum(vec![2, 3, 5, 7, 7, 7, 5], vec![0, 7, 5]),
            17
        );
    }

    // Java: void minimumValueSum3()
    //   assertThat(
    //   new Solution().minimumValueSum(new int[] {1, 2, 3, 4}, new int[] {2}), equalTo(-1));
    #[test]
    fn test_minimum_value_sum3() {
        assert_eq!(Solution::minimum_value_sum(vec![1, 2, 3, 4], vec![2]), -1);
    }
}
