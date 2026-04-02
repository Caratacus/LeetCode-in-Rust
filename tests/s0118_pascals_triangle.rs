// Tests for Problem 0118: Pascal's Triangle
// Java reference: src/test/java/g0101_0200/s0118_pascals_triangle/SolutionTest.java

use leetcode_in_rust::s0118::pascals_triangle::Solution;

#[test]
fn test_generate() {
    let result = Solution::generate(5);
    assert_eq!(result, vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
}

#[test]
fn test_generate2() {
    let result = Solution::generate(1);
    assert_eq!(result, vec![vec![1]]);
}
