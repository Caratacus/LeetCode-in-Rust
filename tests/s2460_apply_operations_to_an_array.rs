// Tests for Problem 2460: Apply Operations to an Array
// Java reference: src/test/java/g2401_2500/s2460_apply_operations_to_an_array/SolutionTest.java

use leetcode_in_rust::s2460::apply_operations_to_an_array::Solution;

#[test]
fn test_apply_operations() {
    assert_eq!(Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]), vec![1, 4, 2, 0, 0, 0]);
}

#[test]
fn test_apply_operations2() {
    assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
}
