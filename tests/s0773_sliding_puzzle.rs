// Tests for Problem 0773: Sliding Puzzle
// Java reference: src/test/java/g0701_0800/s0773_sliding_puzzle/SolutionTest.java

use leetcode_in_rust::s0773::sliding_puzzle::Solution;

#[test]
fn test_sliding_puzzle() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]),
        1
    );
}

#[test]
fn test_sliding_puzzle2() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]),
        -1
    );
}

#[test]
fn test_sliding_puzzle3() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
        5
    );
}
