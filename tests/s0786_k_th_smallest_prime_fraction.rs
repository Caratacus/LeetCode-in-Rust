// Tests for Problem 0786: K-th Smallest Prime Fraction
// Java reference: src/test/java/g0701_0800/s0786_k_th_smallest_prime_fraction/SolutionTest.java

use leetcode_in_rust::s0786::k_th_smallest_prime_fraction::Solution;

#[test]
fn test_kth_smallest_prime_fraction() {
    assert_eq!(Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3), vec![2, 5]);
}

#[test]
fn test_kth_smallest_prime_fraction2() {
    assert_eq!(Solution::kth_smallest_prime_fraction(vec![1, 7], 1), vec![1, 7]);
}
