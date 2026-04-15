// Tests for Problem 3556: Sum of Largest Prime Substrings
// Java reference: src/test/java/g3501_3600/s3556_sum_of_largest_prime_substrings/SolutionTest.java
use leetcode_in_rust::s3556::sum_of_largest_prime_substrings::Solution;
#[test] fn test_sum_of_largest_primes() { assert_eq!(Solution::sum_of_largest_primes("12234".to_string()), 1469i64); }
#[test] fn test_sum_of_largest_primes2() { assert_eq!(Solution::sum_of_largest_primes("111".to_string()), 11i64); }
