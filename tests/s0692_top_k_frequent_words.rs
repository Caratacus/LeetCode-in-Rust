// Tests for Problem 0692: Top K Frequent Words
// Java reference: src/test/java/g0601_0700/s0692_top_k_frequent_words/SolutionTest.java

use leetcode_in_rust::s0692::top_k_frequent_words::Solution;

#[test]
fn test_top_k_frequent() {
    let result = Solution::top_k_frequent(
        vec!["i".to_string(), "love".to_string(), "leetcode".to_string(), "i".to_string(), "love".to_string(), "coding".to_string()],
        2
    );
    assert_eq!(result, vec!["i".to_string(), "love".to_string()]);
}

#[test]
fn test_top_k_frequent2() {
    let result = Solution::top_k_frequent(
        vec![
            "the".to_string(), "day".to_string(), "is".to_string(), "sunny".to_string(),
            "the".to_string(), "the".to_string(), "the".to_string(), "sunny".to_string(),
            "is".to_string(), "is".to_string()
        ],
        4
    );
    assert_eq!(result, vec!["the".to_string(), "is".to_string(), "sunny".to_string(), "day".to_string()]);
}
