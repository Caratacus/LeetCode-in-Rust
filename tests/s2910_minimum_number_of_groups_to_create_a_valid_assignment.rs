// Tests for Problem 2910: Minimum Number of Groups to Create a Valid Assignment
// Java reference: src/test/java/g2901_3000/s2910_minimum_number_of_groups_to_create_a_valid_assignment/SolutionTest.java

use leetcode_in_rust::s2910::minimum_number_of_groups_to_create_a_valid_assignment::Solution;

#[test]
fn test_min_groups_for_valid_assignment() {
    assert_eq!(Solution::min_groups_for_valid_assignment(vec![3, 2, 3, 2, 3]), 2);
}

#[test]
fn test_min_groups_for_valid_assignment2() {
    assert_eq!(Solution::min_groups_for_valid_assignment(vec![10, 10, 10, 3, 1, 1]), 4);
}
