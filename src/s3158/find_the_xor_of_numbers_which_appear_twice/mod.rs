// Problem 3158: find the xor of numbers which appear twice
// #Easy #Array #Hash_Table #Bit_Manipulation #2024_05_30_Time_1_ms_(99.87%)_Space_42.3_MB_(99.40%)

pub struct Solution;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut appeared = [false; 51];
        let mut res = 0;
        for num in nums {
            let idx = num as usize;
            if appeared[idx] {
                res ^= num;
            }
            appeared[idx] = true;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void duplicateNumbersXOR()
    //   assertThat(new Solution().duplicateNumbersXOR(new int[] {1, 2, 1, 3}), equalTo(1));
    #[test]
    fn test_duplicate_numbers_xor() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
    }

    // Java: void duplicateNumbersXOR2()
    //   assertThat(new Solution().duplicateNumbersXOR(new int[] {1, 2, 3}), equalTo(0));
    #[test]
    fn test_duplicate_numbers_xor2() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
    }

    // Java: void duplicateNumbersXOR3()
    //   assertThat(new Solution().duplicateNumbersXOR(new int[] {1, 2, 2, 1}), equalTo(3));
    #[test]
    fn test_duplicate_numbers_xor3() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
    }
}
