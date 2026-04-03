// Tests for Problem 0854: K-Similar Strings
// Java reference: src/test/java/g0801_0900/s0854_k_similar_strings/SolutionTest.java

use leetcode_in_rust::s0854::k_similar_strings::Solution;

#[test]
fn test_k_similarity() {
    assert_eq!(Solution::k_similarity("ab".to_string(), "ba".to_string()), 1);
}

#[test]
fn test_k_similarity2() {
    assert_eq!(Solution::k_similarity("abc".to_string(), "bca".to_string()), 2);
}
