// Tests for Problem 3712: Sum of Elements with Frequency Divisible by K
// Java reference: src/test/java/g3701_3800/s3712_sum_of_elements_with_frequency_divisible_by_k/SolutionTest.java
use leetcode_in_rust::s3712::sum_of_elements_with_frequency_divisible_by_k::Solution;
#[test]
fn test_sum_divisible_by_k() { assert_eq!(Solution::sum_divisible_by_k(vec![1, 2, 2, 3, 3, 3, 3, 4], 2), 16); }
#[test]
fn test_sum_divisible_by_k2() { assert_eq!(Solution::sum_divisible_by_k(vec![1, 2, 3, 4, 5], 2), 0); }
#[test]
fn test_sum_divisible_by_k3() { assert_eq!(Solution::sum_divisible_by_k(vec![4, 4, 4, 1, 2, 3], 3), 12); }
