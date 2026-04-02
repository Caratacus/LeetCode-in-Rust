// Tests for Problem 0052: N-Queens II
// Java reference: src/test/java/g0001_0100/s0052_n_queens_ii/SolutionTest.java

use leetcode_in_rust::s0052::n_queens_ii::Solution;

#[test]
fn test_total_n_queens() {
    assert_eq!(Solution::total_n_queens(4), 2);
}

#[test]
fn test_total_n_queens2() {
    assert_eq!(Solution::total_n_queens(1), 1);
}
