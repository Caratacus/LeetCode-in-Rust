// Tests for Problem 3086: Minimum Moves to Pick K Ones
// Java reference: src/test/java/g3001_3100/s3086_minimum_moves_to_pick_k_ones/SolutionTest.java

use leetcode_in_rust::s3086::minimum_moves_to_pick_k_ones::Solution;

#[test]
fn test_minimum_moves() {
    assert_eq!(
        Solution::minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1),
        3
    );
}

#[test]
fn test_minimum_moves2() {
    assert_eq!(Solution::minimum_moves(vec![0, 0, 0, 0], 2, 3), 4);
}

#[test]
fn test_minimum_moves3() {
    assert_eq!(Solution::minimum_moves(vec![1, 0, 1, 0, 1], 3, 0), 4);
}
