// Tests for Problem 0980: Unique Paths III
// Java reference: src/test/java/g0901_1000/s0980_unique_paths_iii/SolutionTest.java

use leetcode_in_rust::s0980::unique_paths_iii::Solution;

#[test]
fn test_unique_paths_iii() {
    assert_eq!(
        Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
        2
    );
}

#[test]
fn test_unique_paths_iii2() {
    assert_eq!(
        Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
        4
    );
}

#[test]
fn test_unique_paths_iii3() {
    assert_eq!(Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
}
