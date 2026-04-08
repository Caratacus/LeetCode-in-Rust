// Problem 3133: minimum array end
// #Medium #Bit_Manipulation #Math
// #2024_04_27_Time_2_ms(100.00%)  Space 43 MB (75.29%)

pub struct Solution;

impl Solution {
    pub fn min_end(n: i64, x: i32) -> i64 {
        let mut n = n;
        let mut result = x as i64;

        for _ in 1..n {
            let mut temp = result;
            let mut zeros = 0i64;

            // Count trailing zeros
            while temp > 0 && temp & 1 == 0 {
                zeros += 1;
                temp >>= 1;
            }

            if zeros == 0 {
                // No trailing zeros, just increment
                result += 1;
            } else {
                // Set the lowest zero bit
                result |= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_end() {
        assert_eq!(Solution::min_end(3, 4), 7);
    }

    #[test]
    fn test_min_end2() {
        assert_eq!(Solution::min_end(2, 7), 15);
    }
}
