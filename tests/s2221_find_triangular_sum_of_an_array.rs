// Tests for Problem 2221: Find Triangular Sum of an Array
// Java reference: src/test/java/g2201_2300/s2221_find_triangular_sum_of_an_array/SolutionTest.java

use leetcode_in_rust::s2221::find_triangular_sum_of_an_array::Solution;

#[test]
fn test_triangular_sum() {
    assert_eq!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]), 8);
}

#[test]
fn test_triangular_sum2() {
    assert_eq!(Solution::triangular_sum(vec![5]), 5);
}
