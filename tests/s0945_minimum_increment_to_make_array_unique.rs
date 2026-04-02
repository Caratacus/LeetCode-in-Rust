// Tests for Problem 0945: Minimum Increment to Make Array Unique
// Java reference: src/test/java/g0901_1000/s0945_minimum_increment_to_make_array_unique/SolutionTest.java

use leetcode_in_rust::s0945::minimum_increment_to_make_array_unique::Solution;

#[test]
fn test_min_increment_for_unique() {
    assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
}

#[test]
fn test_min_increment_for_unique2() {
    assert_eq!(Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]), 6);
}
