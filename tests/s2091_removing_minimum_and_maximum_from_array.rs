// Tests for Problem 2091: Removing Minimum and Maximum From Array
// Java reference: src/test/java/g2001_2100/s2091_removing_minimum_and_maximum_from_array/SolutionTest.java

use leetcode_in_rust::s2091::removing_minimum_and_maximum_from_array::Solution;

#[test]
fn test_minimum_deletions() {
    assert_eq!(Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6]), 5);
}

#[test]
fn test_minimum_deletions2() {
    assert_eq!(Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5]), 3);
}

#[test]
fn test_minimum_deletions3() {
    assert_eq!(Solution::minimum_deletions(vec![101]), 1);
}
