// Tests for Problem 0149: Max Points on a Line
// Java reference: src/test/java/g0121_0200/s0149_max_points_on_a_line/SolutionTest.java

use leetcode_in_rust::s0149::max_points_on_a_line::Solution;

#[test]
fn test_max_points() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(Solution::max_points(points), 3);
}

#[test]
fn test_max_points2() {
    let points = vec![vec![1, 1], vec![3, 2], vec![5, 3], vec![4, 1], vec![2, 3], vec![1, 4]];
    assert_eq!(Solution::max_points(points), 4);
}
