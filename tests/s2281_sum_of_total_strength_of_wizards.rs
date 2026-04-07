// Tests for Problem 2281: Sum of Total Strength of Wizards
// Java reference: src/test/java/g2201_2300/s2281_sum_of_total_strength_of_wizards/SolutionTest.java

use leetcode_in_rust::s2281::sum_of_total_strength_of_wizards::Solution;

#[test]
fn test_total_strength() {
    assert_eq!(Solution::total_strength(vec![1, 3, 1, 2]), 44);
}

#[test]
fn test_total_strength2() {
    assert_eq!(Solution::total_strength(vec![5, 4, 6]), 213);
}
