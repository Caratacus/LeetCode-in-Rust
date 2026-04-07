// Tests for Problem 2565: Subsequence With the Minimum Score
// Java reference: src/test/java/g2501_2600/s2565_subsequence_with_the_minimum_score/SolutionTest.java

use leetcode_in_rust::s2565::subsequence_with_the_minimum_score::Solution;

#[test]
fn test_minimum_score() {
    assert_eq!(Solution::minimum_score("abacaba".to_string(), "bzaa".to_string()), 1);
}

#[test]
fn test_minimum_score2() {
    assert_eq!(Solution::minimum_score("cde".to_string(), "xyz".to_string()), 3);
}
