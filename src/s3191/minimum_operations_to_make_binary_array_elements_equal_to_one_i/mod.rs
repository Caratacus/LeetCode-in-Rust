// Problem 3191: minimum operations to make binary array elements equal to one i
// #Medium #Array #Bit_Manipulation #Prefix_Sum #Sliding_Window #Queue
// #2024_06_26_Time_6_ms_(99.99%)_Space_57.2_MB_(62.07%)

pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        // Iterate through the array up to the third-last element
        for i in 0..n - 2 {
            // If the current element is 0, perform an operation
            if nums[i] == 0 {
                ans += 1;
                // Flip the current element and the next two elements
                nums[i] = 1;
                nums[i + 1] = if nums[i + 1] == 0 { 1 } else { 0 };
                nums[i + 2] = if nums[i + 2] == 0 { 1 } else { 0 };
            }
        }
        // Check the last two elements if they are 0, return -1 as they cannot be flipped
        for i in n - 2..n {
            if nums[i] == 0 {
                return -1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minOperations()
    //   assertThat(new Solution().minOperations(new int[] {0, 1, 1, 1, 0, 0}), equalTo(3));
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
    }

    // Java: void minOperations2()
    //   assertThat(new Solution().minOperations(new int[] {0, 1, 1, 1}), equalTo(-1));
    #[test]
    fn test_min_operations2() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 1]), -1);
    }
}
