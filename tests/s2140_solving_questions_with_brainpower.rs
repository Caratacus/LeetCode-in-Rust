// Tests for Problem 2140: Solving Questions With Brainpower
// Java reference: src/test/java/g2101_2200/s2140_solving_questions_with_brainpower/SolutionTest.java

use leetcode_in_rust::s2140::solving_questions_with_brainpower::Solution;

#[test]
fn test_most_points() {
    assert_eq!(
        Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
        5
    );
}

#[test]
fn test_most_points2() {
    assert_eq!(
        Solution::most_points(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]),
        7
    );
}
