// Tests for Problem 2870: Minimum Number of Operations to Make Array Empty
// Java reference: src/test/java/g2801_2900/s2870_minimum_number_of_operations_to_make_array_empty/SolutionTest.java

use leetcode_in_rust::s2870::minimum_number_of_operations_to_make_array_empty::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![2, 1, 2, 2, 3, 3]), -1);
}
