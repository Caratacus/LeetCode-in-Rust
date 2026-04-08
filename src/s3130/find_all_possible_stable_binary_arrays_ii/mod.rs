// Problem 3130: find all possible stable binary arrays ii
// #Hard #Dynamic_Programming #PrefixSum #2024_05_02_Time_3_ms_(100.00%)_Space_40.6_MB_(100.00%)

const MOD: i64 = 1_000_000_007;
const N: usize = 1000;

pub struct Solution;

impl Solution {
    fn get_inverse(mut n: i64, modulus: i64) -> i64 {
        let mut p = modulus;
        let mut x: i64 = 1;
        let mut y: i64 = 0;
        while p > 0 {
            let quotient = n / p;
            let remainder = n % p;
            let temp_y = x - quotient * y;
            x = y;
            y = temp_y;
            n = p;
            p = remainder;
        }
        ((x % modulus) + modulus) % modulus
    }

    fn comb(factorial: &[i64], reverse: &[i64], n: usize, k: usize) -> i64 {
        (factorial[n] * reverse[k] % MOD * reverse[n - k]) % MOD
    }

    fn calc(factorial: &[i64], reverse: &[i64], groups: i32, x: i32, limit: i32) -> i64 {
        let mut s: i64 = 0;
        let mut sign: i64 = 1;
        for k in 0..=groups as i64 {
            if k * limit as i64 > (x - groups) as i64 {
                break;
            }
            let k = k as i32;
            s = (s + sign * Self::comb(factorial, reverse, groups as usize, k as usize)
                * Self::comb(factorial, reverse, (x - k * limit) as usize, (groups - 1) as usize))
                % MOD;
            sign *= -1;
        }
        s
    }

    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let mut factorial = vec![0i64; N + 1];
        let mut reverse = vec![0i64; N + 1];
        factorial[0] = 1;
        reverse[0] = 1;
        let mut x: i64 = 1;
        for i in 1..=N {
            x = (x * i as i64) % MOD;
            factorial[i] = x;
            reverse[i] = Self::get_inverse(x, MOD);
        }

        let mut ans: i64 = 0;
        let mut s = vec![0i64; one as usize + 1];
        let n = (zero.min(one) + 1) as usize;
        for groups0 in ((zero + limit - 1) / limit)..=(zero.min(n as i32)) {
            let s0 = Self::calc(&factorial, &reverse, groups0, zero, limit);
            let groups1_start = (groups0 - 1).max((one + limit - 1) / limit);
            let groups1_end = (groups0 + 1).min(one);
            for groups1 in groups1_start..=groups1_end {
                let s1 = if s[groups1 as usize] != 0 {
                    s[groups1 as usize]
                } else {
                    let val = Self::calc(&factorial, &reverse, groups1, one, limit);
                    s[groups1 as usize] = val;
                    val
                };
                let multiplier = if groups1 == groups0 { 2 } else { 1 };
                ans = (ans + s0 * s1 * multiplier) % MOD;
            }
        }
        ((ans + MOD) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void numberOfStableArrays()
    //   assertThat(new Solution().numberOfStableArrays(1, 1, 2), equalTo(2));
    #[test]
    fn test_number_of_stable_arrays() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
    }

    // Java: void numberOfStableArrays2()
    //   assertThat(new Solution().numberOfStableArrays(1, 2, 1), equalTo(1));
    #[test]
    fn test_number_of_stable_arrays2() {
        assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
    }

    // Java: void numberOfStableArrays3()
    //   assertThat(new Solution().numberOfStableArrays(3, 3, 2), equalTo(14));
    #[test]
    fn test_number_of_stable_arrays3() {
        assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
    }
}
