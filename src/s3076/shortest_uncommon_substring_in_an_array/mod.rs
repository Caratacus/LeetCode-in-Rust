// Problem 3076: shortest uncommon substring in an array
// #Medium #Array #String #Hash_Table #Trie #2024_04_16_Time_9_ms_(99.97%)_Space_45.8_MB_(39.57%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        let n = arr.len();
        let mut substring_count: HashMap<String, i32> = HashMap::new();
        let mut substring_to_word: HashMap<String, usize> = HashMap::new();

        // Count all substrings across all words
        for (k, s) in arr.iter().enumerate() {
            let chars: Vec<char> = s.chars().collect();
            let m = chars.len();
            let mut seen: HashMap<String, bool> = HashMap::new();
            for i in 0..m {
                for j in (i + 1)..=m {
                    let sub: String = chars[i..j].iter().collect();
                    if !seen.contains_key(&sub) {
                        seen.insert(sub.clone(), true);
                        *substring_count.entry(sub.clone()).or_insert(0) += 1;
                        if substring_count.get(&sub) == Some(&1) {
                            substring_to_word.insert(sub, k);
                        }
                    }
                }
            }
        }

        // For each word, find the shortest uncommon substring
        let mut ans = Vec::with_capacity(n);
        for (k, s) in arr.iter().enumerate() {
            let chars: Vec<char> = s.chars().collect();
            let m = chars.len();
            let mut result = String::new();
            let mut result_len = m + 1;

            for i in 0..m {
                for j in (i + 1)..=std::cmp::min(m, i + result_len) {
                    let sub: String = chars[i..j].iter().collect();
                    let count = substring_count.get(&sub).copied().unwrap_or(0);
                    if count == 1 && substring_to_word.get(&sub).copied().unwrap_or(usize::MAX) == k {
                        let sub_len = j - i;
                        if sub_len < result_len || (sub_len == result_len && result.as_str() > sub.as_str()) {
                            result = sub;
                            result_len = sub_len;
                        }
                    }
                }
            }
            ans.push(result);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void shortestSubstrings()
    //   assertThat(
    //   new Solution().shortestSubstrings(new String[] {"cab", "ad", "bad", "c"}),
    //   equalTo(new String[] {"ab", "", "ba", ""}));
    #[test]
    fn test_shortest_substrings() {
        assert_eq!(
            Solution::shortest_substrings(vec!["cab".to_string(), "ad".to_string(), "bad".to_string(), "c".to_string()]),
            vec!["ab".to_string(), "".to_string(), "ba".to_string(), "".to_string()]
        );
    }

    // Java: void shortestSubstrings2()
    //   assertThat(
    //   new Solution().shortestSubstrings(new String[] {"abc", "bcd", "abcd"}),
    //   equalTo(new String[] {"", "", "abcd"}));
    #[test]
    fn test_shortest_substrings2() {
        assert_eq!(
            Solution::shortest_substrings(vec!["abc".to_string(), "bcd".to_string(), "abcd".to_string()]),
            vec!["".to_string(), "".to_string(), "abcd".to_string()]
        );
    }
}
