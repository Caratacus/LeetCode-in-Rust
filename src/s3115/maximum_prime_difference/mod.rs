// Problem 3115: Maximum Prime Difference
// #Medium #Array #Math #Number_Theory #2024_04_27_Time_1_ms_(99.91%)_Space_79.5_MB_(32.00%)

pub struct Solution;

impl Solution {
    fn is_not_prime(n: i32) -> bool {
        if n < 2 {
            return true;
        }
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                return true;
            }
            i += 1;
        }
        false
    }

    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut i = 0;
        while i < n && Self::is_not_prime(nums[i]) {
            i += 1;
        }
        let mut j = n as i32 - 1;
        while j >= 0 && Self::is_not_prime(nums[j as usize]) {
            j -= 1;
        }
        j - i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumPrimeDifference()
    //   assertThat(new Solution().maximumPrimeDifference(new int[] {4, 2, 9, 5, 3}), equalTo(3));
    #[test]
    fn test_maximum_prime_difference() {
        assert_eq!(Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
    }

    // Java: void maximumPrimeDifference2()
    //   assertThat(new Solution().maximumPrimeDifference(new int[] {4, 8, 2, 8}), equalTo(0));
    #[test]
    fn test_maximum_prime_difference2() {
        assert_eq!(Solution::maximum_prime_difference(vec![4, 8, 2, 8]), 0);
    }
}
