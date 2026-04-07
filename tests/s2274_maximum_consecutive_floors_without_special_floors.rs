// Tests for Problem 2274: Maximum Consecutive Floors Without Special Floors
// Java reference: src/test/java/g2201_2300/s2274_maximum_consecutive_floors_without_special_floors/SolutionTest.java

use leetcode_in_rust::s2274::maximum_consecutive_floors_without_special_floors::Solution;

#[test]
fn test_max_consecutive() {
    assert_eq!(Solution::max_consecutive(2, 9, vec![4, 6]), 3);
}

#[test]
fn test_max_consecutive2() {
    assert_eq!(Solution::max_consecutive(6, 8, vec![7, 6, 8]), 0);
}
