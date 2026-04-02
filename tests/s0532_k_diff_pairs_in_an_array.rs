// Tests for Problem 0532: K-diff Pairs in an Array
// Java reference: src/test/java/g0501_0600/s0532_k_diff_pairs_in_an_array/SolutionTest.java

use leetcode_in_rust::s0532::k_diff_pairs_in_an_array::Solution;

#[test]
fn test_find_pairs() {
    assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
}

#[test]
fn test_find_pairs2() {
    assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
}

#[test]
fn test_find_pairs3() {
    assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
}
