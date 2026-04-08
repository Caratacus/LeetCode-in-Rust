// Problem 3022: Minimize OR of Remaining Elements Using Operations
// #Hard #Array #Greedy #Bit_Manipulation

pub struct Solution;

impl Solution {
    pub fn min_or_after_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut mask: i32 = 0;

        for j in (0..=30).rev() {
            mask |= 1 << j;
            let mut consecutive_and = mask;
            let mut merge_count = 0;

            for &i in &nums {
                consecutive_and &= i;
                if (consecutive_and | ans) != ans {
                    merge_count += 1;
                } else {
                    consecutive_and = mask;
                }
            }

            if merge_count > k {
                ans |= 1 << j;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minOrAfterOperations()
    //   assertThat(new Solution().minOrAfterOperations(new int[] {3, 5, 3, 2, 7}, 2), equalTo(3));
    #[test]
    fn test_min_or_after_operations() {
        assert_eq!(Solution::min_or_after_operations(vec![3, 5, 3, 2, 7], 2), 3);
    }

    // Java: void minOrAfterOperations2()
    //   assertThat(
    //   new Solution().minOrAfterOperations(new int[] {7, 3, 15, 14, 2, 8}, 4), equalTo(2));
    #[test]
    fn test_min_or_after_operations2() {
        assert_eq!(Solution::min_or_after_operations(vec![7, 3, 15, 14, 2, 8], 4), 2);
    }

    // Java: void minOrAfterOperations3()
    //   assertThat(
    //   new Solution().minOrAfterOperations(new int[] {10, 7, 10, 3, 9, 14, 9, 4}, 1),
    //   equalTo(15));
    #[test]
    fn test_min_or_after_operations3() {
        assert_eq!(Solution::min_or_after_operations(vec![10, 7, 10, 3, 9, 14, 9, 4], 1), 15);
    }
}
