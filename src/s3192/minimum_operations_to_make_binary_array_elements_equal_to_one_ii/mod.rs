// Problem 3192: minimum operations to make binary array elements equal to one ii
// #Medium #Array #Dynamic_Programming #Greedy #2024_06_26_Time_6_ms_(99.64%)_Space_62.9_MB_(17.52%)

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut c = 1;
        for x in nums {
            if x != c {
                a += 1;
                c ^= 1;
            }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minOperations()
    //   assertThat(new Solution().minOperations(new int[] {0, 1, 1, 0, 1}), equalTo(4));
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 0, 1]), 4);
    }

    // Java: void minOperations2()
    //   assertThat(new Solution().minOperations(new int[] {1, 0, 0, 0}), equalTo(1));
    #[test]
    fn test_min_operations2() {
        assert_eq!(Solution::min_operations(vec![1, 0, 0, 0]), 1);
    }
}
