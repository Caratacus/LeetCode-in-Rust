// Tests for Problem 0051: N-Queens
// Java reference: src/test/java/g0001_0100/s0051_n_queens/SolutionTest.java

use leetcode_in_rust::s0051::n_queens::Solution;

#[test]
fn test_solve_n_queens() {
    let result = Solution::solve_n_queens(4);
    assert_eq!(result.len(), 2);
}

#[test]
fn test_solve_n_queens2() {
    let result = Solution::solve_n_queens(1);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], vec!["Q"]);
}
