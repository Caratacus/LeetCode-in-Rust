// Tests for Problem 1092: Shortest Common Supersequence
// Java reference: src/test/java/g1001_1100/s1092_shortest_common_supersequence/SolutionTest.java

use leetcode_in_rust::s1092::shortest_common_supersequence::Solution;

#[test]
fn test_shortest_common_supersequence() {
    assert_eq!(
        Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string()),
        "cabac"
    );
}

#[test]
fn test_shortest_common_supersequence2() {
    assert_eq!(
        Solution::shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string()),
        "aaaaaaaa"
    );
}
