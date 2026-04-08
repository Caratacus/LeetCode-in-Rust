// Problem 3079: find the sum of encrypted integers
// #Easy #Array #Math #2024_04_16_Time_1_ms_(99.95%)_Space_42.7_MB_(75.97%)

pub struct Solution;

impl Solution {
    fn encrypt(x: i32) -> i32 {
        let mut n_digits = 0;
        let mut max = 0;
        let mut x = x;
        while x > 0 {
            max = std::cmp::max(max, x % 10);
            x /= 10;
            n_digits += 1;
        }
        let mut ans = 0;
        for _ in 0..n_digits {
            ans = ans * 10 + max;
        }
        ans
    }

    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        nums.iter().map(|&num| Self::encrypt(num)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sumOfEncryptedInt()
    //   assertThat(new Solution().sumOfEncryptedInt(new int[] {1, 2, 3}), equalTo(6));
    #[test]
    fn test_sum_of_encrypted_int() {
        assert_eq!(Solution::sum_of_encrypted_int(vec![1, 2, 3]), 6);
    }

    // Java: void sumOfEncryptedInt2()
    //   assertThat(new Solution().sumOfEncryptedInt(new int[] {10, 21, 31}), equalTo(66));
    #[test]
    fn test_sum_of_encrypted_int2() {
        assert_eq!(Solution::sum_of_encrypted_int(vec![10, 21, 31]), 66);
    }
}
