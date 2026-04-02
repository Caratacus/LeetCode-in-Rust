// Tests for Problem 0389: Find the Difference
// Java reference: src/test/java/g0301_0400/s0389_find_the_difference/SolutionTest.java

use leetcode_in_rust::s0389::find_the_difference::Solution;

#[test]
fn test_find_the_difference() {
    assert_eq!(Solution::find_the_difference("abcd".to_string(), "abcde".to_string()), 'e');
}

#[test]
fn test_find_the_difference2() {
    assert_eq!(Solution::find_the_difference("".to_string(), "y".to_string()), 'y');
}

#[test]
fn test_find_the_difference3() {
    assert_eq!(Solution::find_the_difference("a".to_string(), "aa".to_string()), 'a');
}

#[test]
fn test_find_the_difference4() {
    assert_eq!(Solution::find_the_difference("ae".to_string(), "aea".to_string()), 'a');
}
