// Tests for Problem 0413: Arithmetic Slices
// Java reference: src/test/java/g0401_0500/s0413_arithmetic_slices/SolutionTest.java

use leetcode_in_rust::s0413::arithmetic_slices::Solution;

#[test]
fn test_number_of_arithmetic_slices() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
}

#[test]
fn test_number_of_arithmetic_slices2() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
}
