// Problem 3153: sum of digit differences of all pairs
// #Medium #Array #Hash_Table #Math #Counting #2024_05_22_Time_12_ms_(100.00%)_Space_62.8_MB_(6.25%)

pub struct Solution;

impl Solution {
    pub fn sum_digit_differences(mut nums: Vec<i32>) -> i64 {
        let mut result: i64 = 0;
        while nums[0] > 0 {
            let mut counts = [0i64; 10];
            for i in 0..nums.len() {
                let digit = (nums[i] % 10) as usize;
                nums[i] = nums[i] / 10;
                result += i as i64 - counts[digit];
                counts[digit] += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sumDigitDifferences()
    //   assertThat(new Solution().sumDigitDifferences(new int[] {13, 23, 12}), equalTo(4L));
    #[test]
    fn test_sum_digit_differences() {
        assert_eq!(Solution::sum_digit_differences(vec![13, 23, 12]), 4);
    }

    // Java: void sumDigitDifferences2()
    //   assertThat(new Solution().sumDigitDifferences(new int[] {10, 10, 10, 10}), equalTo(0L));
    #[test]
    fn test_sum_digit_differences2() {
        assert_eq!(Solution::sum_digit_differences(vec![10, 10, 10, 10]), 0);
    }
}
