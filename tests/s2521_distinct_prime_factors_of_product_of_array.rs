// Tests for Problem 2521: Distinct Prime Factors of Product of Array
// Java reference: src/test/java/g2401_2500/s2521_distinct_prime_factors_of_product_of_array/SolutionTest.java

use leetcode_in_rust::s2521::distinct_prime_factors_of_product_of_array::Solution;

#[test]
fn test_distinct_prime_factors() {
    assert_eq!(Solution::distinct_prime_factors(vec![2, 4, 3, 7, 10, 6]), 4);
}

#[test]
fn test_distinct_prime_factors2() {
    assert_eq!(Solution::distinct_prime_factors(vec![2, 4, 8, 16]), 1);
}
