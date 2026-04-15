// Tests for Problem 3591: Check if Any Element Has Prime Frequency
// Java reference: src/test/java/g3501_3600/s3591_check_if_any_element_has_prime_frequency/SolutionTest.java
use leetcode_in_rust::s3591::check_if_any_element_has_prime_frequency::Solution;
#[test] fn test_check_prime_frequency() { assert_eq!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5, 4]), true); }
#[test] fn test_check_prime_frequency2() { assert_eq!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5]), false); }
#[test] fn test_check_prime_frequency3() { assert_eq!(Solution::check_prime_frequency(vec![2, 2, 2, 4, 4]), true); }
