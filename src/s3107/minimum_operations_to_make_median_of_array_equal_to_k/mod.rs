// Problem 3107: Minimum Operations to Make Median of Array Equal to K
// #Medium #Array #Sorting #Greedy #2024_04_11_Time_28_ms_(98.49%)_Space_61.8_MB_(98.64%)

pub struct Solution;

impl Solution {
    pub fn min_operations_to_make_median_k(mut nums: Vec<i32>, k: i32) -> i64 {
        nums.sort();
        let n = nums.len();
        let median_index = n / 2;
        let result: i64;
        let mut total_elements = 0;
        let mut total_sum: i64 = 0;
        let mut i = median_index;

        if nums[median_index] > k {
            while i < n && nums[i] > k {
                total_elements += 1;
                total_sum += nums[i] as i64;
                if i > 0 {
                    i -= 1;
                } else {
                    break;
                }
            }
        } else if nums[median_index] < k {
            while i < n && nums[i] < k {
                total_elements += 1;
                total_sum += nums[i] as i64;
                i += 1;
            }
        }

        result = (total_sum - (total_elements as i64 * k as i64)).abs();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minOperationsToMakeMedianK()
    //   assertThat(
    //   new Solution().minOperationsToMakeMedianK(new int[] {2, 5, 6, 8, 5}, 4),
    //   equalTo(2L));
    #[test]
    fn test_min_operations_to_make_median_k() {
        assert_eq!(Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 4), 2);
    }

    // Java: void minOperationsToMakeMedianK2()
    //   assertThat(
    //   new Solution().minOperationsToMakeMedianK(new int[] {2, 5, 6, 8, 5}, 7),
    //   equalTo(3L));
    #[test]
    fn test_min_operations_to_make_median_k2() {
        assert_eq!(Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 7), 3);
    }

    // Java: void minOperationsToMakeMedianK3()
    //   assertThat(
    //   new Solution().minOperationsToMakeMedianK(new int[] {1, 2, 3, 4, 5, 6}, 4),
    //   equalTo(0L));
    #[test]
    fn test_min_operations_to_make_median_k3() {
        assert_eq!(Solution::min_operations_to_make_median_k(vec![1, 2, 3, 4, 5, 6], 4), 0);
    }
}
