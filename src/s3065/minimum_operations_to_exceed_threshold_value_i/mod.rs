// Problem 3065: minimum operations to exceed threshold value i
// #Easy #Array #2024_03_31_Time_0_ms_(100.00%)_Space_42.7_MB_(48.42%)

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let count = nums.iter().filter(|&&num| num >= k).count();
        (nums.len() - count) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minOperations()
    //   assertThat(new Solution().minOperations(new int[] {2, 11, 10, 1, 3}, 10), equalTo(3));
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 3);
    }

    // Java: void minOperations2()
    //   assertThat(new Solution().minOperations(new int[] {1, 1, 2, 4, 9}, 1), equalTo(0));
    #[test]
    fn test_min_operations2() {
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 1), 0);
    }

    // Java: void minOperations3()
    //   assertThat(new Solution().minOperations(new int[] {1, 1, 2, 4, 9}, 9), equalTo(4));
    #[test]
    fn test_min_operations3() {
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 9), 4);
    }
}
