// Tests for Problem 0789: Escape The Ghosts
// Java reference: src/test/java/g0701_0800/s0789_escape_the_ghosts/SolutionTest.java

use leetcode_in_rust::s0789::escape_the_ghosts::Solution;

#[test]
fn test_escape_ghosts() {
    assert_eq!(
        Solution::escape_ghosts(vec![vec![1, 0], vec![0, 3]], vec![0, 1]),
        true
    );
}

#[test]
fn test_escape_ghosts2() {
    assert_eq!(
        Solution::escape_ghosts(vec![vec![1, 0]], vec![2, 0]),
        false
    );
}

#[test]
fn test_escape_ghosts3() {
    assert_eq!(
        Solution::escape_ghosts(vec![vec![2, 0]], vec![1, 0]),
        false
    );
}
