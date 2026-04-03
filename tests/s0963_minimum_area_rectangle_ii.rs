// Tests for Problem 0963: Minimum Area Rectangle II
// Java reference: src/test/java/g0901_1000/s0963_minimum_area_rectangle_ii/SolutionTest.java

use leetcode_in_rust::s0963::minimum_area_rectangle_ii::Solution;

#[test]
fn test_min_area_free_rect() {
    let result = Solution::min_area_free_rect(vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]]);
    assert_eq!(result, 2.0);
}

#[test]
fn test_min_area_free_rect2() {
    let result = Solution::min_area_free_rect(vec![vec![0, 1], vec![2, 1], vec![1, 1], vec![1, 0], vec![2, 0]]);
    assert_eq!(result, 1.0);
}

#[test]
fn test_min_area_free_rect3() {
    let result = Solution::min_area_free_rect(vec![vec![0, 3], vec![1, 2], vec![3, 1], vec![1, 3], vec![2, 1]]);
    assert_eq!(result, 0.0);
}
