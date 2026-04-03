// Tests for Problem 0796: Rotate String
// Java reference: src/test/java/g0701_0800/s0796_rotate_string/SolutionTest.java

use leetcode_in_rust::s0796::rotate_string::Solution;

#[test]
fn test_rotate_string() {
    assert_eq!(Solution::rotate_string("abcde".to_string(), "cdeab".to_string()), true);
}

#[test]
fn test_rotate_string2() {
    assert_eq!(Solution::rotate_string("abcde".to_string(), "abced".to_string()), false);
}
