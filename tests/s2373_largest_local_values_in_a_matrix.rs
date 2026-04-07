// Tests for Problem 2373: Largest Local Values in a Matrix
// Java reference: src/test/java/g2301_2400/s2373_largest_local_values_in_a_matrix/SolutionTest.java

use leetcode_in_rust::s2373::largest_local_values_in_a_matrix::Solution;

#[test]
fn test_largest_local() {
    let grid = vec![vec![9, 9, 8, 1], vec![5, 6, 2, 6], vec![8, 2, 6, 4], vec![6, 2, 2, 2]];
    let result = Solution::largest_local(grid);
    let expected = vec![vec![9, 9], vec![8, 6]];
    assert_eq!(result, expected);
}

#[test]
fn test_largest_local2() {
    let grid = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 2, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
    ];
    let result = Solution::largest_local(grid);
    let expected = vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]];
    assert_eq!(result, expected);
}
