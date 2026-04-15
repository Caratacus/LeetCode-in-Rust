// Tests for Problem 3609: Minimum Moves to Reach Target in Grid
// Java reference: src/test/java/g3601_3700/s3609_minimum_moves_to_reach_target_in_grid/SolutionTest.java
use leetcode_in_rust::s3609::minimum_moves_to_reach_target_in_grid::Solution;
#[test]
fn test_min_moves() { assert_eq!(Solution::min_moves(1, 2, 5, 4), 2); }
#[test]
fn test_min_moves2() { assert_eq!(Solution::min_moves(0, 1, 2, 3), 3); }
#[test]
fn test_min_moves3() { assert_eq!(Solution::min_moves(1, 1, 2, 2), -1); }
#[test]
fn test_min_moves4() { assert_eq!(Solution::min_moves(0, 0, 0, 0), 0); }
#[test]
fn test_min_moves5() { assert_eq!(Solution::min_moves(0, 0, 1, 0), -1); assert_eq!(Solution::min_moves(0, 0, 0, 1), -1); }
#[test]
fn test_min_moves6() { assert_eq!(Solution::min_moves(2, 0, 1, 0), -1); }
#[test]
fn test_min_moves7() { assert_eq!(Solution::min_moves(0, 2, 0, 1), -1); }
#[test]
fn test_min_moves8() { assert_eq!(Solution::min_moves(1, 1, 9, 4), -1); }
#[test]
fn test_min_moves9() { assert_eq!(Solution::min_moves(1, 1, 8, 3), -1); }
#[test]
fn test_min_moves10() { assert_eq!(Solution::min_moves(1, 1, 6, 4), -1); }
#[test]
fn test_min_moves11() { assert_eq!(Solution::min_moves(1, 1, 4, 9), -1); }
#[test]
fn test_min_moves12() { assert_eq!(Solution::min_moves(1, 1, 3, 8), -1); }
#[test]
fn test_min_moves13() { assert_eq!(Solution::min_moves(1, 1, 4, 6), -1); }
#[test]
fn test_min_moves14() { assert_eq!(Solution::min_moves(0, 2, 5, 5), -1); }
#[test]
fn test_min_moves15() { assert_eq!(Solution::min_moves(2, 0, 5, 5), -1); }
#[test]
fn test_min_moves16() { assert_eq!(Solution::min_moves(2, 2, 5, 5), -1); }
#[test]
fn test_min_moves17() { assert_eq!(Solution::min_moves(1, 1, 5, 2), -1); }
