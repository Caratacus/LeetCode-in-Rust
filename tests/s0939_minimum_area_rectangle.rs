// Tests for Problem 0939: Minimum Area Rectangle
// Java reference: src/test/java/g0901_1000/s0939_minimum_area_rectangle/SolutionTest.java

use leetcode_in_rust::s0939::minimum_area_rectangle::Solution;

#[test]
fn test_min_area_rect() {
    let result = Solution::min_area_rect(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![2, 2]]);
    assert_eq!(result, 4);
}

#[test]
fn test_min_area_rect2() {
    let result = Solution::min_area_rect(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![4, 1], vec![4, 3]]);
    assert_eq!(result, 2);
}
