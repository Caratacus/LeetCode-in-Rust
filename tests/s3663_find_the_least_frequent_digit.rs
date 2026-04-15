// Tests for Problem 3663: Find the Least Frequent Digit
// Java reference: src/test/java/g3601_3700/s3663_find_the_least_frequent_digit/SolutionTest.java
use leetcode_in_rust::s3663::find_the_least_frequent_digit::Solution;
#[test]
fn test_get_least_frequent_digit() { assert_eq!(Solution::get_least_frequent_digit(1553322), 1); }
#[test]
fn test_get_least_frequent_digit2() { assert_eq!(Solution::get_least_frequent_digit(723344511), 2); }
