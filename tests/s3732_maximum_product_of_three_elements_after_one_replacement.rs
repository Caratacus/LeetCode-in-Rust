// Tests for Problem 3732: Maximum Product of Three Elements After One Replacement
// Java reference: src/test/java/g3701_3800/s3732_maximum_product_of_three_elements_after_one_replacement/SolutionTest.java
use leetcode_in_rust::s3732::maximum_product_of_three_elements_after_one_replacement::Solution;
#[test]
fn test_max_product() { assert_eq!(Solution::max_product(vec![-5, 7, 0]), 3500000i64); }
#[test]
fn test_max_product2() { assert_eq!(Solution::max_product(vec![-4, -2, -1, -3]), 1200000i64); }
#[test]
fn test_max_product3() { assert_eq!(Solution::max_product(vec![0, 10, 0]), 0i64); }
