// Tests for Problem 3423: Maximum Difference Between Adjacent Elements in a Circular Array
// Java reference: src/test/java/g3401_3500/s3423_maximum_difference_between_adjacent_elements_in_a_circular_array/SolutionTest.java

use leetcode_in_rust::s3423::maximum_difference_between_adjacent_elements_in_a_circular_array::Solution;

#[test]
fn test_max_adjacent_distance() {
    assert_eq!(Solution::max_adjacent_distance(vec![1, 2, 4]), 3);
}

#[test]
fn test_max_adjacent_distance2() {
    assert_eq!(Solution::max_adjacent_distance(vec![-5, -10, -5]), 5);
}
