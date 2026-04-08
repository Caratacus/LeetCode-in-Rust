// Problem 3144: minimum substring partition of equal character frequency
// #Medium #String #Hash_Table #Dynamic_Programming #Counting
// #2024_05_15_Time_37_ms_(100.00%)_Space_44.9_MB_(72.95%)

pub struct Solution;

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut dp = vec![n as i32; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            let mut count = [0; 26];
            let mut distinct = 0;
            let mut max_count = 0;
            for j in (0..i).rev() {
                let index = (chars[j] as usize) - ('a' as usize);
                count[index] += 1;
                if count[index] == 1 {
                    distinct += 1;
                }
                if count[index] > max_count {
                    max_count = count[index];
                }
                if max_count * distinct == (i - j) as i32 {
                    dp[i] = dp[i].min(dp[j] + 1);
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_substrings_in_partition() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("fabccddg".to_string()),
            3
        );
    }

    #[test]
    fn test_minimum_substrings_in_partition2() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("abababaccddb".to_string()),
            2
        );
    }
}
