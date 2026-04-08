// Problem 3146: permutation difference between two strings
// #Easy #String #Hash_Table #2024_05_15_Time_1_ms_(100.00%)_Space_42.4_MB_(84.38%)

pub struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut res = [-1; 26];
        let mut sum = 0;
        for (i, c) in s.chars().enumerate() {
            res[(c as usize) - ('a' as usize)] = i as i32;
        }
        for (i, c) in t.chars().enumerate() {
            sum += (res[(c as usize) - ('a' as usize)] - i as i32).abs();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_permutation_difference() {
        assert_eq!(
            Solution::find_permutation_difference("abc".to_string(), "bac".to_string()),
            2
        );
    }

    #[test]
    fn test_find_permutation_difference2() {
        assert_eq!(
            Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string()),
            12
        );
    }
}
