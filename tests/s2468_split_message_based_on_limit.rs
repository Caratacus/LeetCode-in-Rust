// Tests for Problem 2468: Split Message Based on Limit
// Java reference: src/test/java/g2401_2500/s2468_split_message_based_on_limit/SolutionTest.java

use leetcode_in_rust::s2468::split_message_based_on_limit::Solution;

#[test]
fn test_split_message() {
    let result = Solution::split_message("this is really a very awesome message".to_string(), 9);
    assert_eq!(result.len(), 14);
}

#[test]
fn test_split_message2() {
    assert_eq!(
        Solution::split_message("short message".to_string(), 15),
        vec!["short mess<1/2>", "age<2/2>"]
    );
}
