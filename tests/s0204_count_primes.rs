// Tests for Problem 0204: Count Primes
// Java reference: src/test/java/g0201_0300/s0204_count_primes/SolutionTest.java

use leetcode_in_rust::s0204::count_primes::Solution;

#[test]
fn test_count_primes() {
    assert_eq!(Solution::count_primes(10), 4);
}

#[test]
fn test_count_primes2() {
    assert_eq!(Solution::count_primes(0), 0);
}

#[test]
fn test_count_primes3() {
    assert_eq!(Solution::count_primes(1), 0);
}
