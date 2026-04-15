// Tests for Problem 3660: Jump Game IX
// Java reference: src/test/java/g3601_3700/s3660_jump_game_ix/SolutionTest.java
use leetcode_in_rust::s3660::jump_game_ix::Solution;
#[test]
fn test_max_value() { assert_eq!(Solution::max_value(vec![2, 1, 3]), vec![2, 2, 3]); }
#[test]
fn test_max_value2() { assert_eq!(Solution::max_value(vec![2, 3, 1]), vec![3, 3, 3]); }
