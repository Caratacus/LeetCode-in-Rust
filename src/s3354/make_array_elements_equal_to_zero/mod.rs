// Problem 3354: make array elements equal to zero

pub struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left_sum = vec![0i32; n];
        let mut right_sum = vec![0i32; n];
        let mut result = 0;

        for i in 1..n {
            left_sum[i] = left_sum[i - 1] + nums[i - 1];
        }
        for j in (0..n - 1).rev() {
            right_sum[j] = right_sum[j + 1] + nums[j + 1];
        }
        for k in 0..n {
            if nums[k] == 0 {
                let diff = (right_sum[k] - left_sum[k]).abs();
                if diff == 1 {
                    result += 1;
                } else if diff == 0 {
                    result += 2;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countValidSelections()
    //   assertThat(new Solution().countValidSelections(new int[] {1, 0, 2, 0, 3}), equalTo(2));
    #[test]
    fn test_count_valid_selections() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countValidSelections2()
    //   assertThat(
    //   new Solution().countValidSelections(new int[] {2, 3, 4, 0, 4, 1, 0}), equalTo(0));
    #[test]
    fn test_count_valid_selections2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countValidSelections3()
    //   assertThat(
    //   new Solution()
    //   .countValidSelections(new int[] {16, 13, 10, 0, 0, 0, 10, 6, 7, 8, 7}),
    //   equalTo(3));
    #[test]
    fn test_count_valid_selections3() {
        // TODO: 翻译 Java 测试
    }
}
