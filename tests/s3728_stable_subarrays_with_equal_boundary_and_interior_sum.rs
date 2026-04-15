// Tests for Problem 3728: Stable Subarrays with Equal Boundary and Interior Sum
// Java reference: src/test/java/g3701_3800/s3728_stable_subarrays_with_equal_boundary_and_interior_sum/SolutionTest.java
use leetcode_in_rust::s3728::stable_subarrays_with_equal_boundary_and_interior_sum::Solution;
#[test]
fn test_count_stable_subarrays() { assert_eq!(Solution::count_stable_subarrays(vec![9, 3, 3, 3, 9]), 2i64); }
#[test]
fn test_count_stable_subarrays2() { assert_eq!(Solution::count_stable_subarrays(vec![1, 2, 3, 4, 5]), 0i64); }
#[test]
fn test_count_stable_subarrays3() { assert_eq!(Solution::count_stable_subarrays(vec![-4, 4, 0, 0, -8, -4]), 1i64); }
