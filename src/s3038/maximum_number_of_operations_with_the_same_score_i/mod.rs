// Problem 3038: maximum number of operations with the same score i
// #Easy #Array #Simulation #2024_03_04_Time_0_ms_(100.00%)_Space_41.5_MB_(92.21%)

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut c = 1;
        let s = nums[0] + nums[1];
        let l = nums.len() - (if nums.len() % 2 == 0 { 0 } else { 1 });

        let mut i = 2;
        while i < l {
            if nums[i] + nums[i + 1] == s {
                c += 1;
            } else {
                break;
            }
            i += 2;
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxOperations()
    //   assertThat(new Solution().maxOperations(new int[] {3, 2, 1, 4, 5}), equalTo(2));
    #[test]
    fn test_max_operations() {
        let nums = vec![3, 2, 1, 4, 5];
        assert_eq!(Solution::max_operations(nums), 2);
    }

    // Java: void maxOperations2()
    //   assertThat(new Solution().maxOperations(new int[] {3, 2, 6, 1, 4}), equalTo(1));
    #[test]
    fn test_max_operations2() {
        let nums = vec![3, 2, 6, 1, 4];
        assert_eq!(Solution::max_operations(nums), 1);
    }
}
