// Tests for Problem 2528: Maximize the Minimum Powered City
// Java reference: src/test/java/g2401_2500/s2528_maximize_the_minimum_powered_city/SolutionTest.java

use leetcode_in_rust::s2528::maximize_the_minimum_powered_city::Solution;

#[test]
fn test_max_power() {
    assert_eq!(Solution::max_power(vec![1, 2, 4, 5, 0], 1, 2), 5);
}

#[test]
fn test_max_power2() {
    assert_eq!(Solution::max_power(vec![4, 4, 4, 4], 0, 3), 4);
}
