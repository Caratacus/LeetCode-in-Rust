// Tests for Problem 2027: Minimum Moves to Convert String
// Java reference: src/test/java/g2001_2100/s2027_minimum_moves_to_convert_string/SolutionTest.java

use leetcode_in_rust::s2027::minimum_moves_to_convert_string::Solution;

#[test]
fn test_minimum_moves() {
    assert_eq!(Solution::minimum_moves("XXX".to_string()), 1);
}

#[test]
fn test_minimum_moves2() {
    assert_eq!(Solution::minimum_moves("XXOX".to_string()), 2);
}

#[test]
fn test_minimum_moves3() {
    assert_eq!(Solution::minimum_moves("OOOO".to_string()), 0);
}
