// Problem 3405: count the number of arrays with k matching adjacent elements
// #Hard #Math #Combinatorics

const MOD: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as usize;
        let m = m as i64;
        let k = k as usize;
        let mut f = vec![0i64; n + 1];
        f[0] = 1;
        f[1] = 1;
        for i in 2..=n {
            f[i] = (f[i - 1] * i as i64) % MOD;
        }
        let ans = Self::comb(n - 1, k, &f);
        let ans = ans * m % MOD;
        let ans = ans * Self::ex(m - 1, (n - k - 1) as i64) % MOD;
        ans as i32
    }

    fn ex(b: i64, mut e: i64) -> i64 {
        let mut ans = 1i64;
        let mut b = b;
        while e > 0 {
            if e % 2 == 1 {
                ans = (ans * b) % MOD;
            }
            b = (b * b) % MOD;
            e >>= 1;
        }
        ans
    }

    fn comb(n: usize, r: usize, f: &[i64]) -> i64 {
        let ff = |x: i64| -> i64 { Self::ex(x, MOD - 2) };
        f[n] * ff(f[r]) % MOD * ff(f[n - r]) % MOD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countGoodArrays()
    //   assertThat(new Solution().countGoodArrays(3, 2, 1), equalTo(4));
    #[test]
    fn test_count_good_arrays() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countGoodArrays2()
    //   assertThat(new Solution().countGoodArrays(4, 2, 2), equalTo(6));
    #[test]
    fn test_count_good_arrays2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void countGoodArrays3()
    //   assertThat(new Solution().countGoodArrays(5, 2, 0), equalTo(2));
    #[test]
    fn test_count_good_arrays3() {
        // TODO: 翻译 Java 测试
    }
}
