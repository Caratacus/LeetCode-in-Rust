// Tests for Problem 3688: Bitwise OR of Even Numbers in an Array
// Java reference: src/test/java/g3601_3700/s3688_bitwise_or_of_even_numbers_in_an_array/SolutionTest.java
use leetcode_in_rust::s3688::bitwise_or_of_even_numbers_in_an_array::Solution;
#[test]
fn test_even_number_bitwise_ors() { assert_eq!(Solution::even_number_bitwise_o_rs(vec![1, 2, 3, 4, 5, 6]), 6); }
#[test]
fn test_even_number_bitwise_ors2() { assert_eq!(Solution::even_number_bitwise_o_rs(vec![7, 9, 11]), 0); }
