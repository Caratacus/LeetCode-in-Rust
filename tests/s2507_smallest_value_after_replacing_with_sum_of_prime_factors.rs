// Tests for Problem 2507: Smallest Value After Replacing With Sum of Prime Factors
// Java reference: src/test/java/g2401_2500/s2507_smallest_value_after_replacing_with_sum_of_prime_factors/SolutionTest.java

use leetcode_in_rust::s2507::smallest_value_after_replacing_with_sum_of_prime_factors::Solution;

#[test]
fn test_smallest_value() {
    assert_eq!(Solution::smallest_value(15), 5);
}

#[test]
fn test_smallest_value2() {
    assert_eq!(Solution::smallest_value(3), 3);
}
