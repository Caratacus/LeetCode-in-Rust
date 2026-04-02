// Tests for Problem 0119: Pascal's Triangle II
// Java reference: src/test/java/g0101_0200/s0119_pascals_triangle_ii/SolutionTest.java

use leetcode_in_rust::s0119::pascals_triangle_ii::Solution;

#[test]
fn test_get_row() {
    let result = Solution::get_row(3);
    assert_eq!(result, vec![1, 3, 3, 1]);
}

#[test]
fn test_get_row2() {
    let result = Solution::get_row(0);
    assert_eq!(result, vec![1]);
}

#[test]
fn test_get_row3() {
    let result = Solution::get_row(1);
    assert_eq!(result, vec![1, 1]);
}
