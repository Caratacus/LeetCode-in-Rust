// Tests for Problem 0329: Longest Increasing Path in a Matrix
// Java reference: src/test/java/g0301_0400/s0329_longest_increasing_path_in_a_matrix/SolutionTest.java

use leetcode_in_rust::s0329::longest_increasing_path_in_a_matrix::Solution;

#[test]
fn test_longest_increasing_path() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
}

#[test]
fn test_longest_increasing_path2() {
    let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
}

#[test]
fn test_longest_increasing_path3() {
    let matrix = vec![vec![1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 1);
}
