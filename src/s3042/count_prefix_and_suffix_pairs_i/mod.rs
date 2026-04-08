// Problem 3042: Count Prefix and Suffix Pairs I
// #Easy #Array #String #Trie #Hash_Function #String_Matching #Rolling_Hash
// #Big_O_Time_O(n^2*m)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        let n = words.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if Self::is_prefix_and_suffix(&words[i], &words[j]) {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_prefix_and_suffix(str1: &str, str2: &str) -> bool {
        let m = str1.len();
        let n = str2.len();
        if m > n {
            return false;
        }
        let bytes1 = str1.as_bytes();
        let bytes2 = str2.as_bytes();

        // Check prefix
        for i in 0..m {
            if bytes1[i] != bytes2[i] {
                return false;
            }
        }
        // Check suffix
        for i in 0..m {
            if bytes1[i] != bytes2[n - m + i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_prefix_suffix_pairs() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec![
                "a".to_string(),
                "aba".to_string(),
                "ababa".to_string(),
                "aa".to_string()
            ]),
            4
        );
    }

    #[test]
    fn test_count_prefix_suffix_pairs2() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec![
                "pa".to_string(),
                "papa".to_string(),
                "ma".to_string(),
                "mama".to_string()
            ]),
            2
        );
    }

    #[test]
    fn test_count_prefix_suffix_pairs3() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec!["abab".to_string(), "ab".to_string()]),
            0
        );
    }
}
