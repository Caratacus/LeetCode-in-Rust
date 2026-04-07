// Tests for Problem 2467: Most Profitable Path in a Tree
// Java reference: src/test/java/g2401_2500/s2467_most_profitable_path_in_a_tree/SolutionTest.java

use leetcode_in_rust::s2467::most_profitable_path_in_a_tree::Solution;

#[test]
fn test_most_profitable_path() {
    assert_eq!(
        Solution::most_profitable_path(vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]], 3, vec![-7280, -5395, -4296, -4267]),
        -7280
    );
}

#[test]
fn test_most_profitable_path2() {
    assert_eq!(
        Solution::most_profitable_path(vec![vec![0, 1]], 1, vec![-7280, 0]),
        -7280
    );
}
