// Tests for Problem 0978: Longest Turbulent Subarray
// Java reference: src/test/java/g0901_1000/s0978_longest_turbulent_subarray/SolutionTest.java

use leetcode_in_rust::s0978::longest_turbulent_subarray::Solution;

#[test]
fn test_max_turbulence_size() {
    assert_eq!(Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);
}

#[test]
fn test_max_turbulence_size2() {
    assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
}

#[test]
fn test_max_turbulence_size3() {
    assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
}
