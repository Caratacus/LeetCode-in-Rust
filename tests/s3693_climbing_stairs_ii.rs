// Tests for Problem 3693: Climbing Stairs II
// Java reference: src/test/java/g3601_3700/s3693_climbing_stairs_ii/SolutionTest.java
use leetcode_in_rust::s3693::climbing_stairs_ii::Solution;
#[test]
fn test_climb_stairs() { assert_eq!(Solution::climb_stairs(4, vec![1, 2, 3, 4]), 13); }
#[test]
fn test_climb_stairs2() { assert_eq!(Solution::climb_stairs(4, vec![5, 1, 6, 2]), 11); }
#[test]
fn test_climb_stairs3() { assert_eq!(Solution::climb_stairs(3, vec![9, 8, 3]), 12); }
