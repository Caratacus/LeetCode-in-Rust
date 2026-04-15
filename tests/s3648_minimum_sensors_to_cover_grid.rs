// Tests for Problem 3648: Minimum Sensors to Cover Grid
// Java reference: src/test/java/g3601_3700/s3648_minimum_sensors_to_cover_grid/SolutionTest.java
use leetcode_in_rust::s3648::minimum_sensors_to_cover_grid::Solution;
#[test]
fn test_min_sensors() { assert_eq!(Solution::min_sensors(5, 5, 1), 4); }
#[test]
fn test_min_sensors2() { assert_eq!(Solution::min_sensors(2, 2, 2), 1); }
#[test]
fn test_min_sensors3() { assert_eq!(Solution::min_sensors(9, 9, 1), 9); }
#[test]
fn test_min_sensors4() { assert_eq!(Solution::min_sensors(10, 10, 1), 16); }
#[test]
fn test_min_sensors5() { assert_eq!(Solution::min_sensors(2, 2, 1), 1); }
#[test]
fn test_min_sensors6() { assert_eq!(Solution::min_sensors(1, 10, 1), 4); }
#[test]
fn test_large_k() { assert_eq!(Solution::min_sensors(5, 5, 10), 1); }
#[test]
fn test_k_zero() { assert_eq!(Solution::min_sensors(3, 3, 0), 9); }
