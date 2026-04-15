// Tests for Problem 3700: Number of Zigzag Arrays II
// Java reference: src/test/java/g3601_3700/s3700_number_of_zigzag_arrays_ii/SolutionTest.java
use leetcode_in_rust::s3700::number_of_zigzag_arrays_ii::Solution;
#[test]
fn test_zig_zag_arrays() { assert_eq!(Solution::zig_zag_arrays(3, 4, 5), 2); }
#[test]
fn test_zig_zag_arrays2() { assert_eq!(Solution::zig_zag_arrays(3, 1, 3), 10); }
