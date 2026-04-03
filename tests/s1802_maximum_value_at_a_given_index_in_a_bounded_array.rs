// Tests for Problem 1802: Maximum Value at a Given Index in a Bounded Array
// Java reference: src/test/java/g1801_1900/s1802_maximum_value_at_a_given_index_in_a_bounded_array/SolutionTest.java

use leetcode_in_rust::s1802::maximum_value_at_a_given_index_in_a_bounded_array::Solution;

#[test]
fn test_max_value() {
    assert_eq!(Solution::max_value(4, 2, 6), 2);
}

#[test]
fn test_max_value2() {
    assert_eq!(Solution::max_value(6, 1, 10), 3);
}
