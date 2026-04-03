// Tests for Problem 1505: Minimum Possible Integer After at Most K Adjacent Swaps on Digits
// Java reference: src/test/java/g1501_1600/s1505_minimum_possible_integer_after_at_most_k_adjacent_swaps_on_digits/SolutionTest.java

use leetcode_in_rust::s1505::minimum_possible_integer_after_at_most_k_adjacent_swaps_on_digits::Solution;

#[test]
fn test_min_integer() {
    assert_eq!(Solution::min_integer("4321".to_string(), 4), "1342");
}

#[test]
fn test_min_integer2() {
    assert_eq!(Solution::min_integer("100".to_string(), 1), "010");
}

#[test]
fn test_min_integer3() {
    assert_eq!(Solution::min_integer("36789".to_string(), 1000), "36789");
}
