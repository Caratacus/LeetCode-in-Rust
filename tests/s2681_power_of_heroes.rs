// Tests for Problem 2681: Power of Heroes
// Java reference: src/test/java/g2601_2700/s2681_power_of_heroes/SolutionTest.java

use leetcode_in_rust::s2681::power_of_heroes::Solution;

#[test]
fn test_sum_of_power() {
    assert_eq!(Solution::sum_of_power(vec![2, 1, 4]), 141);
}

#[test]
fn test_sum_of_power2() {
    assert_eq!(Solution::sum_of_power(vec![1, 1, 1]), 7);
}
