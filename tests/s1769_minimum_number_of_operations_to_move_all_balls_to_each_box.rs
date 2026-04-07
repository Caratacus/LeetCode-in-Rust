// Tests for Problem 1769: Minimum Number of Operations to Move All Balls to Each Box
// Java reference: src/test/java/g1701_1800/s1769_minimum_number_of_operations_to_move_all_balls_to_each_box/SolutionTest.java

use leetcode_in_rust::s1769::minimum_number_of_operations_to_move_all_balls_to_each_box::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations("110".to_string()), vec![1, 1, 3]);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations("001011".to_string()), vec![11, 8, 5, 4, 3, 4]);
}
