// Tests for Problem 2170: Minimum Operations to Make the Array Alternating
// Java reference: src/test/java/g2101_2200/s2170_minimum_operations_to_make_the_array_alternating/SolutionTest.java

use leetcode_in_rust::s2170::minimum_operations_to_make_the_array_alternating::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(Solution::minimum_operations(vec![3, 1, 3, 2, 4, 3]), 3);
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(Solution::minimum_operations(vec![1, 2, 2, 2, 2]), 2);
}
