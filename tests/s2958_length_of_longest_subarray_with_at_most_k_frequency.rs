// Tests for Problem 2958: Length of Longest Subarray With at Most K Frequency
// Java reference: src/test/java/g2901_3000/s2958_length_of_longest_subarray_with_at_most_k_frequency/SolutionTest.java

use leetcode_in_rust::s2958::length_of_longest_subarray_with_at_most_k_frequency::Solution;

#[test]
fn test_max_subarray_length() {
    assert_eq!(Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2), 6);
}

#[test]
fn test_max_subarray_length2() {
    assert_eq!(Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1), 2);
}

#[test]
fn test_max_subarray_length3() {
    assert_eq!(Solution::max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4), 4);
}
