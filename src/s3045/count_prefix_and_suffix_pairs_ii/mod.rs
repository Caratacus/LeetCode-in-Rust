// Problem 3045: Count Prefix and Suffix Pairs II
// #Hard #Array #String #Trie #Hash_Function #String_Matching #Rolling_Hash
// #Big_O_Time_O(n^2*m)_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let mut ans: i64 = 0;
        let n = words.len();
        let mut visited = vec![false; n];

        for i in 0..n {
            let p = &words[i];
            if !visited[i] {
                let mut found: i64 = 1;
                for j in (i + 1)..n {
                    let s = &words[j];
                    if s.len() >= p.len() && s.starts_with(p) && s.ends_with(p) {
                        ans += found;
                    }
                    if p == s {
                        found += 1;
                        visited[j] = true;
                    }
                }
            }
        }
        ans
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
