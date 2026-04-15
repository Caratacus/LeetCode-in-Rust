// Tests for Problem 3670: Maximum Product of Two Integers with No Common Bits
// Java reference: src/test/java/g3601_3700/s3670_maximum_product_of_two_integers_with_no_common_bits/SolutionTest.java
use leetcode_in_rust::s3670::maximum_product_of_two_integers_with_no_common_bits::Solution;
#[test]
fn test_max_product() { assert_eq!(Solution::max_product(vec![1, 2, 3, 4, 5, 6, 7]), 12i64); }
#[test]
fn test_max_product2() { assert_eq!(Solution::max_product(vec![4, 5, 6]), 0i64); }
#[test]
fn test_max_product3() { assert_eq!(Solution::max_product(vec![64, 8, 32]), 2048i64); }
