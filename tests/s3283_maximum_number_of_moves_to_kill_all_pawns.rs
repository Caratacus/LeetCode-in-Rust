// Tests for Problem 3283: Maximum Number of Moves to Kill All Pawns
// Java reference: src/test/java/g3201_3300/s3283_maximum_number_of_moves_to_kill_all_pawns/SolutionTest.java

use leetcode_in_rust::s3283::maximum_number_of_moves_to_kill_all_pawns::Solution;

#[test]
fn test_max_moves() {
    assert_eq!(Solution::max_moves(1, 1, vec![vec![0, 0]]), 4);
}

#[test]
fn test_max_moves2() {
    assert_eq!(Solution::max_moves(0, 2, vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 8);
}

#[test]
fn test_max_moves3() {
    assert_eq!(Solution::max_moves(0, 0, vec![vec![1, 2], vec![2, 4]]), 3);
}
