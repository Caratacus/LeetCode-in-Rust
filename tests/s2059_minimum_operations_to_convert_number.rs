// Tests for Problem 2059: Minimum Operations to Convert Number
// Java reference: src/test/java/g2001_2100/s2059_minimum_operations_to_convert_number/SolutionTest.java

use leetcode_in_rust::s2059::minimum_operations_to_convert_number::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(Solution::minimum_operations(vec![2, 4, 12], 2, 12), 2);
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(Solution::minimum_operations(vec![3, 5, 7], 0, -4), 2);
}

#[test]
fn test_minimum_operations3() {
    assert_eq!(Solution::minimum_operations(vec![2, 8, 16], 0, 1), -1);
}
