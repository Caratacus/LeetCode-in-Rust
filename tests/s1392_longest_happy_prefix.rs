// Tests for Problem 1392: Longest Happy Prefix
// Java reference: src/test/java/g1301_1400/s1392_longest_happy_prefix/SolutionTest.java

use leetcode_in_rust::s1392::longest_happy_prefix::Solution;

#[test]
fn test_longest_prefix() {
    assert_eq!(Solution::longest_prefix("level".to_string()), "l");
}

#[test]
fn test_longest_prefix2() {
    assert_eq!(Solution::longest_prefix("ababab".to_string()), "abab");
}

#[test]
fn test_longest_prefix3() {
    assert_eq!(Solution::longest_prefix("babbb".to_string()), "b");
}
