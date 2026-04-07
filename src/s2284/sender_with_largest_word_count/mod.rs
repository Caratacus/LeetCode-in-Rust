// Problem 2284: sender with largest word count

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let mut word_count: HashMap<&str, usize> = HashMap::new();
        for (msg, sender) in messages.iter().zip(senders.iter()) {
            let count = msg.split_whitespace().count();
            *word_count.entry(sender.as_str()).or_insert(0) += count;
        }
        word_count
            .into_iter()
            .max_by(|a, b| {
                match a.1.cmp(&b.1) {
                    std::cmp::Ordering::Equal => a.0.cmp(b.0), // Return lexicographically larger
                    other => other,
                }
            })
            .map(|(s, _)| s.to_string())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void largestWordCount()
    //   assertThat(
    //   new Solution()
    //   .largestWordCount(
    //   new String[] {
    //   "Hello userTwooo",
    //   ... (6 more lines)
    #[test]
    fn test_largest_word_count() {
        // TODO: 翻译 Java 测试
    }

    // Java: void largestWordCount2()
    //   assertThat(
    //   new Solution()
    //   .largestWordCount(
    //   new String[] {
    //   "How is leetcode for everyone",
    //   ... (4 more lines)
    #[test]
    fn test_largest_word_count2() {
        // TODO: 翻译 Java 测试
    }
}
