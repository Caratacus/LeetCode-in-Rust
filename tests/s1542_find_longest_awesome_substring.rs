// Tests for Problem 1542: Find Longest Awesome Substring
// Java reference: src/test/java/g1501_1600/s1542_find_longest_awesome_substring/SolutionTest.java

use leetcode_in_rust::s1542::find_longest_awesome_substring::Solution;

#[test]
fn test_longest_awesome() {
    assert_eq!(Solution::longest_awesome("3242415".to_string()), 5);
}

#[test]
fn test_longest_awesome2() {
    assert_eq!(Solution::longest_awesome("12345678".to_string()), 1);
}

#[test]
fn test_longest_awesome3() {
    assert_eq!(Solution::longest_awesome("213123".to_string()), 6);
}
