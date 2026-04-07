// Tests for Problem 2357: Make Array Zero by Subtracting Equal Amounts
// Java reference: src/test/java/g2301_2400/s2357_make_array_zero_by_subtracting_equal_amounts/SolutionTest.java

use leetcode_in_rust::s2357::make_array_zero_by_subtracting_equal_amounts::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(Solution::minimum_operations(vec![1, 5, 0, 3, 5]), 3);
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(Solution::minimum_operations(vec![0]), 0);
}
