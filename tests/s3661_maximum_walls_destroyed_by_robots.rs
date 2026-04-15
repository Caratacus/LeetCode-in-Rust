// Tests for Problem 3661: Maximum Walls Destroyed by Robots
// Java reference: src/test/java/g3601_3700/s3661_maximum_walls_destroyed_by_robots/SolutionTest.java
use leetcode_in_rust::s3661::maximum_walls_destroyed_by_robots::Solution;
#[test]
fn test_max_walls() { assert_eq!(Solution::max_walls(vec![4], vec![3], vec![1, 10]), 1); }
#[test]
fn test_max_walls2() { assert_eq!(Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]), 3); }
#[test]
fn test_max_walls3() { assert_eq!(Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]), 0); }
#[test]
fn test_max_walls4() { assert_eq!(Solution::max_walls(vec![5], vec![3], vec![]), 0); }
#[test]
fn test_max_walls5() { assert_eq!(Solution::max_walls(vec![5], vec![3], vec![2, 4, 5, 6, 8]), 3); }
#[test]
fn test_max_walls6() { assert_eq!(Solution::max_walls(vec![10], vec![2], vec![7, 8, 9, 10, 11, 13]), 3); }
#[test]
fn test_max_walls7() { assert_eq!(Solution::max_walls(vec![5, 15], vec![2, 2], vec![4, 5, 6, 14, 15, 16]), 4); }
#[test]
fn test_max_walls8() { assert_eq!(Solution::max_walls(vec![5, 8], vec![5, 5], vec![2, 4, 5, 6, 7, 8, 9, 10]), 6); }
#[test]
fn test_max_walls9() { assert_eq!(Solution::max_walls(vec![3, 10, 20], vec![2, 3, 4], vec![1, 2, 3, 4, 5, 8, 10, 12, 17, 19, 20, 22]), 8); }
