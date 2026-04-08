// Problem 3008: find beautiful indices in the given array ii
// #Hard #String #Binary_Search #Two_Pointers #Hash_Function #String_Matching #Rolling_Hash
// #2024_02_27_Time_36_ms_(99.66%)_Space_67.9_MB_(99.32%)

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        let lps_a = Self::get_lps(&a);
        let lps_b = Self::get_lps(&b);
        let mut ans: Vec<i32> = Vec::new();
        let mut matches_a: VecDeque<i32> = VecDeque::new();
        let n = s.len();
        let a_len = a.len();
        let b_len = b.len();
        let s_bytes = s.as_bytes();
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut i = 0usize;
        let mut j = 0usize;
        while i < n {
            if s_bytes[i] == a_bytes[j] {
                i += 1;
                j += 1;
            } else {
                if j == 0 {
                    i += 1;
                } else {
                    j = lps_a[j - 1] as usize;
                }
            }
            if j == a_len {
                let a_start = (i - a_len) as i32;
                matches_a.push_back(a_start);
                j = lps_a[a_len - 1] as usize;
            }
        }
        i = 0;
        j = 0;
        while i < n && !matches_a.is_empty() {
            if s_bytes[i] == b_bytes[j] {
                i += 1;
                j += 1;
            } else {
                if j == 0 {
                    i += 1;
                } else {
                    j = lps_b[j - 1] as usize;
                }
            }
            if j == b_len {
                let b_start = (i - b_len) as i32;
                j = lps_b[b_len - 1] as usize;
                while !matches_a.is_empty() && b_start - matches_a[0] > k {
                    matches_a.pop_front();
                }
                while !matches_a.is_empty() && (matches_a[0] - b_start).abs() <= k {
                    ans.push(matches_a.pop_front().unwrap());
                }
            }
        }
        ans
    }

    fn get_lps(s: &str) -> Vec<i32> {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut lps = vec![0i32; n];
        let mut i = 1usize;
        let mut prev_lps = 0usize;
        while i < n {
            if s_bytes[i] == s_bytes[prev_lps] {
                prev_lps += 1;
                lps[i] = prev_lps as i32;
                i += 1;
            } else {
                if prev_lps == 0 {
                    lps[i] = 0;
                    i += 1;
                } else {
                    prev_lps = lps[prev_lps - 1] as usize;
                }
            }
        }
        lps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beautiful_indices() {
        assert_eq!(
            Solution::beautiful_indices(
                "isawsquirrelnearmysquirrelhouseohmy".to_string(),
                "my".to_string(),
                "squirrel".to_string(),
                15
            ),
            vec![16, 33]
        );
    }

    #[test]
    fn test_beautiful_indices2() {
        assert_eq!(
            Solution::beautiful_indices("abcd".to_string(), "a".to_string(), "a".to_string(), 4),
            vec![0]
        );
    }
}
