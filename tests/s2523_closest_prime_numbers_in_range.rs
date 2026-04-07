// Tests for Problem 2523: Closest Prime Numbers in Range
// Java reference: src/test/java/g2401_2500/s2523_closest_prime_numbers_in_range/SolutionTest.java

use leetcode_in_rust::s2523::closest_prime_numbers_in_range::Solution;

#[test]
fn test_closest_primes() {
    assert_eq!(Solution::closest_primes(10, 19), vec![11, 13]);
}

#[test]
fn test_closest_primes2() {
    assert_eq!(Solution::closest_primes(4, 6), vec![-1, -1]);
}
