// Tests for Problem 2457: Minimum Addition to Make Integer Beautiful
// Java reference: src/test/java/g2401_2500/s2457_minimum_addition_to_make_integer_beautiful/SolutionTest.java

use leetcode_in_rust::s2457::minimum_addition_to_make_integer_beautiful::Solution;

#[test]
fn test_make_integer_beautiful() {
    assert_eq!(Solution::make_integer_beautiful(16, 6), 4);
}

#[test]
fn test_make_integer_beautiful2() {
    assert_eq!(Solution::make_integer_beautiful(467, 6), 33);
}

#[test]
fn test_make_integer_beautiful3() {
    assert_eq!(Solution::make_integer_beautiful(1, 1), 0);
}
