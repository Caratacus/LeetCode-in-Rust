// Tests for Problem 3551: Minimum Swaps to Sort by Digit Sum
// Java reference: src/test/java/g3501_3600/s3551_minimum_swaps_to_sort_by_digit_sum/SolutionTest.java

use leetcode_in_rust::s3551::minimum_swaps_to_sort_by_digit_sum::Solution;

#[test]
fn test_min_swaps() { assert_eq!(Solution::min_swaps(vec![37, 100]), 1); }
#[test]
fn test_min_swaps2() { assert_eq!(Solution::min_swaps(vec![22, 14, 33, 7]), 0); }
#[test]
fn test_min_swaps3() { assert_eq!(Solution::min_swaps(vec![18, 43, 34, 16]), 2); }
