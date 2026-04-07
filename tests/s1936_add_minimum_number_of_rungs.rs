// Tests for Problem 1936: Add Minimum Number of Rungs
// Java reference: src/test/java/g1901_2000/s1936_add_minimum_number_of_rungs/SolutionTest.java

use leetcode_in_rust::s1936::add_minimum_number_of_rungs::Solution;

#[test]
fn test_add_rungs() {
    assert_eq!(Solution::add_rungs(vec![3, 6, 8, 10], 3), 0);
}

#[test]
fn test_add_rungs2() {
    assert_eq!(Solution::add_rungs(vec![3, 4, 6, 7], 2), 1);
}
