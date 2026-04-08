// Problem 3091: apply operations to make sum of array greater than or equal to k
// #Medium #Math #Greedy #Enumeration #2024_04_18_Time_0_ms_(100.00%)_Space_40.6_MB_(62.55%)

pub struct Solution;

impl Solution {
    pub fn min_operations(k: i32) -> i32 {
        let a = (k as f64).sqrt() as i32;
        a + (k - 1) / a - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minOperations()
    //   assertThat(new Solution().minOperations(11), equalTo(5));
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(11), 5);
    }

    // Java: void minOperations2()
    //   assertThat(new Solution().minOperations(1), equalTo(0));
    #[test]
    fn test_min_operations2() {
        assert_eq!(Solution::min_operations(1), 0);
    }
}
