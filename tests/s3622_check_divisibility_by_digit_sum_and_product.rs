// Tests for Problem 3622: Check Divisibility by Digit Sum and Product
// Java reference: src/test/java/g3601_3700/s3622_check_divisibility_by_digit_sum_and_product/SolutionTest.java
use leetcode_in_rust::s3622::check_divisibility_by_digit_sum_and_product::Solution;
#[test]
fn test_check_divisibility() { assert_eq!(Solution::check_divisibility(99), true); }
#[test]
fn test_check_divisibility2() { assert_eq!(Solution::check_divisibility(23), false); }
