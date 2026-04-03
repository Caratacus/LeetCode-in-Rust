// Tests for Problem 1316: Distinct Echo Substrings
// Java reference: src/test/java/g1301_1400/s1316_distinct_echo_substrings/SolutionTest.java

use leetcode_in_rust::s1316::distinct_echo_substrings::Solution;

#[test]
fn test_distinct_echo_substrings() {
    let result = Solution::distinct_echo_substrings("abcabcabc".to_string());
    assert_eq!(result, 3);
}

#[test]
fn test_distinct_echo_substrings2() {
    let result = Solution::distinct_echo_substrings("leetcodeleetcode".to_string());
    assert_eq!(result, 2);
}
