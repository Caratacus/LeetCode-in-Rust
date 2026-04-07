// Tests for Problem 2875: Minimum Size Subarray in Infinite Array
// Java reference: src/test/java/g2801_2900/s2875_minimum_size_subarray_in_infinite_array/SolutionTest.java

use leetcode_in_rust::s2875::minimum_size_subarray_in_infinite_array::Solution;

#[test]
fn test_min_size_subarray() {
    assert_eq!(Solution::min_size_subarray(vec![1, 2, 3], 5), 2);
}

#[test]
fn test_min_size_subarray2() {
    assert_eq!(Solution::min_size_subarray(vec![1, 1, 1, 2, 3], 4), 2);
}

#[test]
fn test_min_size_subarray3() {
    assert_eq!(Solution::min_size_subarray(vec![2, 4, 6, 8], 3), -1);
}

#[test]
fn test_min_size_subarray4() {
    assert_eq!(Solution::min_size_subarray(vec![0], 1), -1);
}
