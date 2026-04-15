// Tests for Problem 3658: GCD of Odd and Even Sums
// Java reference: src/test/java/g3601_3700/s3658_gcd_of_odd_and_even_sums/SolutionTest.java
use leetcode_in_rust::s3658::gcd_of_odd_and_even_sums::Solution;
#[test]
fn test_gcd_of_odd_even_sums() { assert_eq!(Solution::gcd_of_odd_even_sums(4), 4); }
#[test]
fn test_gcd_of_odd_even_sums2() { assert_eq!(Solution::gcd_of_odd_even_sums(5), 5); }
#[test]
fn test_gcd_of_odd_even_sums3() { assert_eq!(Solution::gcd_of_odd_even_sums(42), 42); }
#[test]
fn test_gcd_of_odd_even_sums4() { assert_eq!(Solution::gcd_of_odd_even_sums(-42), 42); }
#[test]
fn test_gcd_of_odd_even_sums5() { assert_eq!(Solution::gcd_of_odd_even_sums(0), 0); }
