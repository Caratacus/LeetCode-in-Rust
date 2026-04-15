// Tests for Problem 3568: Minimum Moves to Clean the Classroom
// Java reference: src/test/java/g3501_3600/s3568_minimum_moves_to_clean_the_classroom/SolutionTest.java
use leetcode_in_rust::s3568::minimum_moves_to_clean_the_classroom::Solution;
#[test] fn test_min_moves() { assert_eq!(Solution::min_moves(vec!["S.".to_string(), "XL".to_string()], 2), 2); }
#[test] fn test_min_moves2() { assert_eq!(Solution::min_moves(vec!["LS".to_string(), "RL".to_string()], 4), 3); }
#[test] fn test_min_moves3() { assert_eq!(Solution::min_moves(vec!["L.S".to_string(), "RXL".to_string()], 3), -1); }
