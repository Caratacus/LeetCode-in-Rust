// Problem 3026: Maximum Good Subarray Sum
// #Medium #Array #Hash_Table #Prefix_Sum

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(input: Vec<i32>, target_difference: i32) -> i64 {
        let mut value_to_min_prefix_sum: HashMap<i32, i64> = HashMap::new();
        let mut prefix_sum: i64 = 0;
        let mut max_subarray_sum: i64 = i64::MIN;

        for &value in &input {
            let val = value as i64;

            // Check value + target_difference
            if let Some(&min_prefix) = value_to_min_prefix_sum.get(&(value + target_difference)) {
                max_subarray_sum = max_subarray_sum.max(prefix_sum + val - min_prefix);
            }

            // Check value - target_difference
            if let Some(&min_prefix) = value_to_min_prefix_sum.get(&(value - target_difference)) {
                max_subarray_sum = max_subarray_sum.max(prefix_sum + val - min_prefix);
            }

            // Update min prefix sum for current value
            let entry = value_to_min_prefix_sum.entry(value).or_insert(i64::MAX);
            if *entry > prefix_sum {
                *entry = prefix_sum;
            }

            prefix_sum += val;
        }

        if max_subarray_sum != i64::MIN {
            max_subarray_sum
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumSubarraySum()
    //   assertThat(
    //   new Solution().maximumSubarraySum(new int[] {1, 2, 3, 4, 5, 6}, 1), equalTo(11L));
    #[test]
    fn test_maximum_subarray_sum() {
        assert_eq!(Solution::maximum_subarray_sum(vec![1, 2, 3, 4, 5, 6], 1), 11);
    }

    // Java: void maximumSubarraySum2()
    //   assertThat(new Solution().maximumSubarraySum(new int[] {-1, 3, 2, 4, 5}, 3), equalTo(11L));
    #[test]
    fn test_maximum_subarray_sum2() {
        assert_eq!(Solution::maximum_subarray_sum(vec![-1, 3, 2, 4, 5], 3), 11);
    }

    // Java: void maximumSubarraySum3()
    //   assertThat(new Solution().maximumSubarraySum(new int[] {-1, -2, -3, -4}, 2), equalTo(-6L));
    #[test]
    fn test_maximum_subarray_sum3() {
        assert_eq!(Solution::maximum_subarray_sum(vec![-1, -2, -3, -4], 2), -6);
    }
}
