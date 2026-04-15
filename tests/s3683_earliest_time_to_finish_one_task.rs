// Tests for Problem 3683: Earliest Time to Finish One Task
// Java reference: src/test/java/g3601_3700/s3683_earliest_time_to_finish_one_task/SolutionTest.java
use leetcode_in_rust::s3683::earliest_time_to_finish_one_task::Solution;
#[test]
fn test_earliest_time() { assert_eq!(Solution::earliest_time(vec![vec![1, 6], vec![2, 3]]), 5); }
#[test]
fn test_earliest_time2() { assert_eq!(Solution::earliest_time(vec![vec![100, 100], vec![100, 100], vec![100, 100]]), 200); }
#[test]
fn test_earliest_time3() { assert_eq!(Solution::earliest_time(vec![vec![1, 6]]), 7); }
