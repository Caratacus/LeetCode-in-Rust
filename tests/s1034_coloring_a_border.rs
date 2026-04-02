// Tests for Problem 1034: Coloring A Border
// Java reference: src/test/java/g1001_1100/s1034_coloring_a_border/SolutionTest.java

use leetcode_in_rust::s1034::coloring_a_border::Solution;

#[test]
fn test_color_border() {
    assert_eq!(
        Solution::color_border(vec![vec![1, 1], vec![1, 2]], 0, 0, 3),
        vec![vec![3, 3], vec![3, 2]]
    );
}

#[test]
fn test_color_border2() {
    assert_eq!(
        Solution::color_border(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3),
        vec![vec![1, 3, 3], vec![2, 3, 3]]
    );
}

#[test]
fn test_color_border3() {
    assert_eq!(
        Solution::color_border(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]
    );
}
