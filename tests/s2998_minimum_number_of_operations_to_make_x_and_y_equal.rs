// Tests for Problem 2998: Minimum Number of Operations to Make X and Y Equal
// Java reference: src/test/java/g2901_3000/s2998_minimum_number_of_operations_to_make_x_and_y_equal/SolutionTest.java

use leetcode_in_rust::s2998::minimum_number_of_operations_to_make_x_and_y_equal::Solution;

#[test]
fn test_minimum_operations_to_make_equal() {
    assert_eq!(Solution::minimum_operations_to_make_equal(26, 1), 3);
}

#[test]
fn test_minimum_operations_to_make_equal2() {
    assert_eq!(Solution::minimum_operations_to_make_equal(54, 2), 4);
}
