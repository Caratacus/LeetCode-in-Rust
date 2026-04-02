// Tests for Problem 1044: Longest Duplicate Substring
// Java reference: src/test/java/g1001_1100/s1044_longest_duplicate_substring/SolutionTest.java

use leetcode_in_rust::s1044::longest_duplicate_substring::Solution;

#[test]
fn test_longest_dup_substring() {
    assert_eq!(Solution::longest_dup_substring("banana".to_string()), "ana");
}

#[test]
fn test_longest_dup_substring2() {
    assert_eq!(Solution::longest_dup_substring("abcd".to_string()), "");
}
