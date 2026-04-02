// Tests for Problem 0274: H-Index
// Java reference: src/test/java/g0201_0300/s0274_h_index/SolutionTest.java

use leetcode_in_rust::s0274::h_index::Solution;

#[test]
fn test_h_index() {
    assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
}

#[test]
fn test_h_index2() {
    assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
}
