// Tests for Problem 3733: Minimum Time to Complete All Deliveries
// Java reference: src/test/java/g3701_3800/s3733_minimum_time_to_complete_all_deliveries/SolutionTest.java
use leetcode_in_rust::s3733::minimum_time_to_complete_all_deliveries::Solution;
#[test]
fn test_minimum_time() { assert_eq!(Solution::minimum_time(vec![3, 1], vec![2, 3]), 5i64); }
#[test]
fn test_minimum_time2() { assert_eq!(Solution::minimum_time(vec![1, 3], vec![2, 2]), 7i64); }
#[test]
fn test_minimum_time3() { assert_eq!(Solution::minimum_time(vec![2, 1], vec![3, 4]), 3i64); }
