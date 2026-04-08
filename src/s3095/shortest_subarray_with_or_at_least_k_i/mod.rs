// Problem 3095: shortest subarray with or at least k i
// #Easy #Array #Bit_Manipulation #Sliding_Window
// #2024_04_18_Time_1_ms_(98.94%)_Space_42.3_MB_(57.80%)

pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut max_l = n + 1;

        for i in 0..n {
            let mut val = 0;
            for j in i..n {
                val |= nums[j];
                if val >= k {
                    max_l = std::cmp::min(max_l, j - i + 1);
                }
            }
        }

        if max_l == n + 1 { -1 } else { max_l as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumSubarrayLength()
    //   assertThat(new Solution().minimumSubarrayLength(new int[] {1, 2, 3}, 2), equalTo(1));
    #[test]
    fn test_minimum_subarray_length() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
    }

    // Java: void minimumSubarrayLength2()
    //   assertThat(new Solution().minimumSubarrayLength(new int[] {2, 1, 8}, 10), equalTo(3));
    #[test]
    fn test_minimum_subarray_length2() {
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
    }

    // Java: void minimumSubarrayLength3()
    //   assertThat(new Solution().minimumSubarrayLength(new int[] {1, 2}, 0), equalTo(1));
    #[test]
    fn test_minimum_subarray_length3() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
