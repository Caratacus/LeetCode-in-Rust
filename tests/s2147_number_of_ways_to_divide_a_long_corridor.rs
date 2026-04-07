// Tests for Problem 2147: Number of Ways to Divide a Long Corridor
// Java reference: src/test/java/g2101_2200/s2147_number_of_ways_to_divide_a_long_corridor/SolutionTest.java

use leetcode_in_rust::s2147::number_of_ways_to_divide_a_long_corridor::Solution;

#[test]
fn test_number_of_ways() {
    assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
}

#[test]
fn test_number_of_ways2() {
    assert_eq!(Solution::number_of_ways("PPSPSP".to_string()), 1);
}

#[test]
fn test_number_of_ways3() {
    assert_eq!(Solution::number_of_ways("S".to_string()), 0);
}
