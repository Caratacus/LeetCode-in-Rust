// Problem 3035: maximum palindromes after operations
// #Medium #Array #String #Hash_Table #Sorting #Greedy #Counting
// #2024_03_01_Time_4_ms_(99.13%)_Space_44.9_MB_(90.28%)

pub struct Solution;

impl Solution {
    pub fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
        let mut char_count = vec![0; 26];
        let mut len_count = vec![0; 101];
        let mut singles = 0;
        let mut pairs = 0;
        let mut ans = 0;

        for word in &words {
            for c in word.chars() {
                char_count[(c as usize) - ('a' as usize)] += 1;
            }
            len_count[word.len()] += 1;
        }

        for j in char_count.iter() {
            singles += j % 2;
            pairs += j / 2;
        }

        for i in 1..=100 {
            if len_count[i] > 0 {
                if i % 2 == 0 {
                    while len_count[i] > 0 && pairs > 0 {
                        pairs -= i as i32 / 2;
                        if pairs >= 0 {
                            ans += 1;
                        }
                        len_count[i] -= 1;
                    }
                } else {
                    while len_count[i] > 0 && (i == 1 || pairs > 0) {
                        if singles == 0 {
                            singles += 2;
                            pairs -= 1;
                        }
                        singles -= 1;
                        pairs -= (i as i32 - 1) / 2;
                        if pairs >= 0 {
                            ans += 1;
                        }
                        len_count[i] -= 1;
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
    fn test_max_palindromes_after_operations() {
        assert_eq!(
            Solution::max_palindromes_after_operations(vec!["abbb".to_string(), "ba".to_string(), "aa".to_string()]),
            3
        );
    }

    #[test]
    fn test_max_palindromes_after_operations2() {
        assert_eq!(
            Solution::max_palindromes_after_operations(vec!["abc".to_string(), "ab".to_string()]),
            2
        );
    }

    #[test]
    fn test_max_palindromes_after_operations3() {
        assert_eq!(
            Solution::max_palindromes_after_operations(vec!["cd".to_string(), "ef".to_string(), "a".to_string()]),
            1
        );
    }
}
