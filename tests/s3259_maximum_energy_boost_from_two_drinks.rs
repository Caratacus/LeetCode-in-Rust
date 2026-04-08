// Tests for Problem 3259: Maximum Energy Boost From Two Drinks
// Java reference: src/test/java/g3201_3300/s3259_maximum_energy_boost_from_two_drinks/SolutionTest.java

use leetcode_in_rust::s3259::maximum_energy_boost_from_two_drinks::Solution;

#[test]
fn test_max_energy_boost() {
    assert_eq!(Solution::max_energy_boost(vec![1, 3, 1], vec![3, 1, 1]), 5);
}

#[test]
fn test_max_energy_boost2() {
    assert_eq!(Solution::max_energy_boost(vec![4, 1, 1], vec![1, 1, 3]), 7);
}
