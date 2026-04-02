// Tests for Problem 0961: N-Repeated Element in Size 2N Array
// Java reference: src/test/java/g0901_1000/s0961_n_repeated_element_in_size_2n_array/SolutionTest.java

use leetcode_in_rust::s0961::n_repeated_element_in_size_2n_array::Solution;

#[test]
fn test_repeated_n_times() {
    assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
}

#[test]
fn test_repeated_n_times2() {
    assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
}

#[test]
fn test_repeated_n_times3() {
    assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
}
