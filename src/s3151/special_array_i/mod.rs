// Problem 3151: special array i
// #Easy #Array #2024_05_22_Time_0_ms_(100.00%)_Space_43.2_MB_(51.16%)

pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if nums[i - 1] % 2 == 1 && nums[i] % 2 == 1 {
                return false;
            }
            if nums[i - 1] % 2 == 0 && nums[i] % 2 == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isArraySpecial()
    //   assertThat(new Solution().isArraySpecial(new int[] {1}), equalTo(true));
    #[test]
    fn test_is_array_special() {
        assert_eq!(Solution::is_array_special(vec![1]), true);
    }

    // Java: void isArraySpecial2()
    //   assertThat(new Solution().isArraySpecial(new int[] {2, 1, 4}), equalTo(true));
    #[test]
    fn test_is_array_special2() {
        assert_eq!(Solution::is_array_special(vec![2, 1, 4]), true);
    }

    // Java: void isArraySpecial3()
    //   assertThat(new Solution().isArraySpecial(new int[] {4, 3, 1, 6}), equalTo(false));
    #[test]
    fn test_is_array_special3() {
        assert_eq!(Solution::is_array_special(vec![4, 3, 1, 6]), false);
    }

    // Java: void isArraySpecial4()
    //   assertThat(new Solution().isArraySpecial(new int[] {2, 10}), equalTo(false));
    #[test]
    fn test_is_array_special4() {
        assert_eq!(Solution::is_array_special(vec![2, 10]), false);
    }
}
