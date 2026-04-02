// Tests for Problem 1036: Escape a Large Maze
// Java reference: src/test/java/g1001_1100/s1036_escape_a_large_maze/SolutionTest.java

use leetcode_in_rust::s1036::escape_a_large_maze::Solution;

#[test]
fn test_is_escape_possible() {
    assert_eq!(
        Solution::is_escape_possible(vec![vec![0, 1], vec![1, 0]], vec![0, 0], vec![0, 2]),
        false
    );
}

#[test]
fn test_is_escape_possible2() {
    assert_eq!(
        Solution::is_escape_possible(vec![], vec![0, 0], vec![999999, 999999]),
        true
    );
}

#[test]
fn test_is_escape_possible3() {
    assert_eq!(
        Solution::is_escape_possible(vec![vec![1, 1]], vec![0, 0], vec![1, 1]),
        true
    );
}
