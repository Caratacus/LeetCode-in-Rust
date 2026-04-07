// Tests for Problem 2400: Number of Ways to Reach a Position After Exactly K Steps
// Java reference: src/test/java/g2301_2400/s2400_number_of_ways_to_reach_a_position_after_exactly_k_steps/SolutionTest.java

use leetcode_in_rust::s2400::number_of_ways_to_reach_a_position_after_exactly_k_steps::Solution;

#[test]
fn test_number_of_ways() {
    assert_eq!(Solution::number_of_ways(1, 2, 3), 3);
}

#[test]
fn test_number_of_ways2() {
    assert_eq!(Solution::number_of_ways(2, 5, 10), 0);
}

#[test]
fn test_number_of_ways3() {
    assert_eq!(Solution::number_of_ways(1, 10, 3), 0);
}

#[test]
fn test_number_of_ways4() {
    assert_eq!(Solution::number_of_ways(1, 1000, 999), 1);
}

#[test]
fn test_number_of_ways5() {
    assert_eq!(Solution::number_of_ways(272, 270, 6), 15);
}
