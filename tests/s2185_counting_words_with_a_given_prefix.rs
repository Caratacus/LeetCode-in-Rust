// Tests for Problem 2185: Counting Words With a Given Prefix
// Java reference: src/test/java/g2101_2200/s2185_counting_words_with_a_given_prefix/SolutionTest.java

use leetcode_in_rust::s2185::counting_words_with_a_given_prefix::Solution;

#[test]
fn test_prefix_count() {
    assert_eq!(
        Solution::prefix_count(
            vec!["pay".to_string(), "attention".to_string(), "practice".to_string(), "attend".to_string()],
            "at".to_string()
        ),
        2
    );
}

#[test]
fn test_prefix_count2() {
    assert_eq!(
        Solution::prefix_count(
            vec!["leetcode".to_string(), "win".to_string(), "loops".to_string(), "success".to_string()],
            "code".to_string()
        ),
        0
    );
}
