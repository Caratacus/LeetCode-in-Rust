// Problem 3190: find minimum operations to make all elements divisible by three
// #Easy #Array #Math #2024_06_26_Time_0_ms_(100.00%)_Space_41.6_MB_(78.56%)

pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for num in nums {
            if num % 3 != 0 {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumOperations()
    //   assertThat(new Solution().minimumOperations(new int[] {1, 2, 3, 4}), equalTo(3));
    #[test]
    fn test_minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
    }

    // Java: void minimumOperations2()
    //   assertThat(new Solution().minimumOperations(new int[] {3, 6, 9}), equalTo(0));
    #[test]
    fn test_minimum_operations2() {
        assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
    }
}
