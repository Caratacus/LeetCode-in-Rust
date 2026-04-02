// Tests for Problem 0525: Contiguous Array
// Java reference: src/test/java/g0501_0600/s0525_contiguous_array/SolutionTest.java

use leetcode_in_rust::s0525::contiguous_array::Solution;

#[test]
fn test_find_max_length() {
    assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
}

#[test]
fn test_find_max_length2() {
    assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
}
