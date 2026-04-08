// Tests for Problem 3095: Shortest Subarray With OR at Least K I
// Java reference: src/test/java/g3001_3100/s3095_shortest_subarray_with_or_at_least_k_i/SolutionTest.java

use leetcode_in_rust::s3095::shortest_subarray_with_or_at_least_k_i::Solution;

#[test]
fn test_minimum_subarray_length() {
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
}

#[test]
fn test_minimum_subarray_length2() {
    assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
}

#[test]
fn test_minimum_subarray_length3() {
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
}
