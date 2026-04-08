// Problem 3097: shortest subarray with or at least k ii
// #Medium #Array #Bit_Manipulation #Sliding_Window
// #2024_04_19_Time_7_ms_(98.43%)_Space_70.2_MB_(74.25%)

pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;

        if nums[0] >= k {
            return 1;
        }

        let mut res = i32::MAX;
        for i in 1..n {
            if nums[i] >= k {
                return 1;
            }
            let mut j = i as i32 - 1;
            while j >= 0 && (nums[i] | nums[j as usize]) != nums[j as usize] {
                nums[j as usize] |= nums[i];
                if nums[j as usize] >= k {
                    res = std::cmp::min(res, i as i32 - j + 1);
                }
                j -= 1;
            }
        }

        if res == i32::MAX { -1 } else { res }
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
