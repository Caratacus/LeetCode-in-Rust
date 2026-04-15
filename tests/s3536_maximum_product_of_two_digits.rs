// Tests for Problem 3536: Maximum Product of Two Digits
// Java reference: src/test/java/g3501_3600/s3536_maximum_product_of_two_digits/SolutionTest.java

use leetcode_in_rust::s3536::maximum_product_of_two_digits::Solution;

#[test]
fn test_max_product() { assert_eq!(Solution::max_product(31), 3); }

#[test]
fn test_max_product2() { assert_eq!(Solution::max_product(22), 4); }

#[test]
fn test_max_product3() { assert_eq!(Solution::max_product(124), 8); }

#[test]
fn test_max_product4() { assert_eq!(Solution::max_product(453), 20); }

#[test]
fn test_max_product5() { assert_eq!(Solution::max_product(437), 28); }

#[test]
fn test_max_product6() { assert_eq!(Solution::max_product(724), 28); }
