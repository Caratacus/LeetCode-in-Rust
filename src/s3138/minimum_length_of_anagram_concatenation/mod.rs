// Problem 3138: minimum length of anagram concatenation
// #Medium #String #Hash_Table #Counting #2024_05_07_Time_4 ms(84.18%)  Space 45.3 MB (81.03%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let n = s.len();
        let mut sq = vec![0i64; n];
        let bytes = s.as_bytes();

        for i in 0..n {
            let ch = bytes[i] as i64;
            if i == 0 {
                sq[i] = ch * ch;
            } else {
                sq[i] = sq[i - 1] + ch * ch;
            }
        }

        let factors = Self::get_all_factors(n);

        for &factor in &factors {
            if factor == 1 {
                if sq[0] * (n as i64) == sq[n - 1] {
                    return 1;
                }
            } else {
                let sum = sq[factor - 1];
                let mut start = 0i64;
                let mut valid = true;
                for i in (factor - 1)..n {
                    if start + sum != sq[i] {
                        valid = false;
                        break;
                    }
                    start += sum;
                    if i == n - 1 {
                        return factor as i32;
                    }
                }
                if valid && (n - factor) % factor == 0 {
                    return factor as i32;
                }
            }
        }
        n as i32 - 1
    }

    fn get_all_factors(n: usize) -> Vec<usize> {
        let mut factors = Vec::new();
        let sqrt_n = (n as f64).sqrt() as usize;
        for i in 1..=sqrt_n {
            if n % i == 0 {
                factors.push(i);
                factors.push(n / i);
            }
        }
        factors.sort();
        factors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_anagram_length() {
        assert_eq!(Solution::min_anagram_length(String::from("abba")), 2);
    }

    #[test]
    fn test_min_anagram_length2() {
        assert_eq!(Solution::min_anagram_length(String::from("cdef")), 4);
    }
}
