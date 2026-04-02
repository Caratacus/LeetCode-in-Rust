// Tests for Problem 0992: Subarrays with K Different Integers
// Java reference: src/test/java/g0901_1000/s0992_subarrays_with_k_different_integers/SolutionTest.java

use leetcode_in_rust::s0992::subarrays_with_k_different_integers::Solution;

#[test]
fn test_subarrays_with_k_distinct() {
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
}

#[test]
fn test_subarrays_with_k_distinct2() {
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
}
