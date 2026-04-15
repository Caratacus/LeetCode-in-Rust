// Tests for Problem 3639: Minimum Time to Activate String
// Java reference: src/test/java/g3601_3700/s3639_minimum_time_to_activate_string/SolutionTest.java
use leetcode_in_rust::s3639::minimum_time_to_activate_string::Solution;
#[test]
fn test_min_time() { assert_eq!(Solution::min_time("abc".to_string(), vec![1, 0, 2], 2), 0); }
#[test]
fn test_min_time2() { assert_eq!(Solution::min_time("cat".to_string(), vec![0, 2, 1], 6), 2); }
#[test]
fn test_min_time3() { assert_eq!(Solution::min_time("xy".to_string(), vec![0, 1], 4), -1); }
