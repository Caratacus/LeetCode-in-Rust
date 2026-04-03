// Tests for Problem 1610: Maximum Number of Visible Points
// Java reference: src/test/java/g1601_1700/s1610_maximum_number_of_visible_points/SolutionTest.java

use leetcode_in_rust::s1610::maximum_number_of_visible_points::Solution;

#[test]
fn test_visible_points() {
    assert_eq!(
        Solution::visible_points(
            vec![vec![2, 1], vec![2, 2], vec![3, 3], vec![1, 1]],
            90,
            vec![1, 1]
        ),
        4
    );
}

#[test]
fn test_visible_points2() {
    assert_eq!(
        Solution::visible_points(
            vec![vec![2, 1], vec![2, 2], vec![3, 4], vec![1, 1]],
            90,
            vec![1, 1]
        ),
        3
    );
}

#[test]
fn test_visible_points3() {
    assert_eq!(
        Solution::visible_points(vec![vec![1, 0], vec![2, 1]], 13, vec![1, 1]),
        1
    );
}
