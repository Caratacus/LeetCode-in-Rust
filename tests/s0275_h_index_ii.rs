// Tests for Problem 0275: H-Index II
// Java reference: src/test/java/g0201_0300/s0275_h_index_ii/SolutionTest.java

use leetcode_in_rust::s0275::h_index_ii::Solution;

#[test]
fn test_h_index() {
    assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
}

#[test]
fn test_h_index2() {
    assert_eq!(Solution::h_index(vec![1, 2, 100]), 2);
}
