// Tests for Problem 0383: Ransom Note
// Java reference: src/test/java/g0301_0400/s0383_ransom_note/SolutionTest.java

use leetcode_in_rust::s0383::ransom_note::Solution;

#[test]
fn test_can_construct() {
    assert_eq!(Solution::can_construct("a".to_string(), "b".to_string()), false);
}

#[test]
fn test_can_construct2() {
    assert_eq!(Solution::can_construct("aa".to_string(), "ab".to_string()), false);
}

#[test]
fn test_can_construct3() {
    assert_eq!(Solution::can_construct("aa".to_string(), "aab".to_string()), true);
}
