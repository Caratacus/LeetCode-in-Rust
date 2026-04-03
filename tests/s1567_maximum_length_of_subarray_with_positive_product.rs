// Tests for Problem 1567: Maximum Length of Subarray With Positive Product
// Java reference: src/test/java/g1501_1600/s1567_maximum_length_of_subarray_with_positive_product/SolutionTest.java

use leetcode_in_rust::s1567::maximum_length_of_subarray_with_positive_product::Solution;

#[test]
fn test_get_max_len() {
    assert_eq!(Solution::get_max_len(vec![1, -2, -3, 4]), 4);
}

#[test]
fn test_get_max_len2() {
    assert_eq!(Solution::get_max_len(vec![0, 1, -2, -3, -4]), 3);
}

#[test]
fn test_get_max_len3() {
    assert_eq!(Solution::get_max_len(vec![-1, -2, -3, 0, 1]), 2);
}
