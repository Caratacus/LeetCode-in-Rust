// Problem 3179: find the n th value after k seconds
// #Medium #Array #Math #Simulation #Prefix_Sum #Combinatorics
// #2024_06_14_Time_2_ms_(99.86%)_Space_40.9_MB_(85.18%)

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        Self::combination(k + n - 1, k)
    }

    fn combination(a: i32, b: i32) -> i32 {
        let mut numerator: i64 = 1;
        let mut denominator: i64 = 1;
        for i in 0..b {
            numerator = (numerator * (a - i) as i64) % MOD;
            denominator = (denominator * (i + 1) as i64) % MOD;
        }
        // Calculate the modular inverse of denominator
        let denominator_inverse = Self::power(denominator, (MOD - 2) as i32);
        ((numerator * denominator_inverse) % MOD) as i32
    }

    // Function to calculate power
    fn power(mut x: i64, mut y: i32) -> i64 {
        let mut result = 1;
        while y > 0 {
            if y % 2 == 1 {
                result = (result * x) % MOD;
            }
            y >>= 1;
            x = (x * x) % MOD;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void valueAfterKSeconds()
    //   assertThat(new Solution().valueAfterKSeconds(4, 5), equalTo(56));
    #[test]
    fn test_value_after_k_seconds() {
        assert_eq!(Solution::value_after_k_seconds(4, 5), 56);
    }

    // Java: void valueAfterKSeconds2()
    //   assertThat(new Solution().valueAfterKSeconds(5, 3), equalTo(35));
    #[test]
    fn test_value_after_k_seconds2() {
        assert_eq!(Solution::value_after_k_seconds(5, 3), 35);
    }
}
