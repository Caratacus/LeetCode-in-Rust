// Tests for Problem 3179: Find the N th Value after K Seconds
// Java reference: src/test/java/g3101_3200/s3179_find_the_n_th_value_after_k_seconds/SolutionTest.java

use leetcode_in_rust::s3179::find_the_n_th_value_after_k_seconds::Solution;
const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        combination(k + n - 1, k)
    }
    fn combination(a: i32, b: i32) -> i64 {
        let numerator = 1;
        let denominator = 1;
        for i in 0..b {
            numerator = (numerator * (a - i)) % MOD;
            denominator = (denominator * (i + 1)) % MOD;
        }
        let denominator_inverse = power(denominator, mod - 2);
        ((numerator * denominator_inverse) % mod)
    }
    fn power(mut x: i64, y: i32) -> i64 {
        let result = 1;
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