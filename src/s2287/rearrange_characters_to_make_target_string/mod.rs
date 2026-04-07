// Problem 2287: rearrange characters to make target string

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut s_count: HashMap<char, usize> = HashMap::new();
        let mut t_count: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *s_count.entry(c).or_insert(0) += 1;
        }
        for c in target.chars() {
            *t_count.entry(c).or_insert(0) += 1;
        }
        t_count
            .iter()
            .map(|(c, &need)| s_count.get(c).unwrap_or(&0) / need)
            .min()
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void rearrangeCharacters()
    //   assertThat(new Solution().rearrangeCharacters("abcba", "abc"), equalTo(1));
    #[test]
    fn test_rearrange_characters() {
        // TODO: 翻译 Java 测试
    }

    // Java: void rearrangeCharacters2()
    //   assertThat(new Solution().rearrangeCharacters("abbaccaddaeea", "aaaaa"), equalTo(1));
    #[test]
    fn test_rearrange_characters2() {
        // TODO: 翻译 Java 测试
    }
}
