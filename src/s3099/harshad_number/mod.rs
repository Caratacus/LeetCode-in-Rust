// Problem 3099: harshad number
// #Easy #Math #2024_04_19_Time_0_ms_(100.00%)_Space_40.9_MB_(7.00%)

pub struct Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut sum = 0;
        let mut temp = x;
        while temp != 0 {
            let digit = temp % 10;
            sum += digit;
            temp /= 10;
        }
        if sum != 0 && x % sum == 0 {
            return sum;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void sumOfTheDigitsOfHarshadNumber()
    //   assertThat(new Solution().sumOfTheDigitsOfHarshadNumber(18), equalTo(9));
    #[test]
    fn test_sum_of_the_digits_of_harshad_number() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
    }

    // Java: void sumOfTheDigitsOfHarshadNumber2()
    //   assertThat(new Solution().sumOfTheDigitsOfHarshadNumber(23), equalTo(-1));
    #[test]
    fn test_sum_of_the_digits_of_harshad_number2() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);
    }
}
