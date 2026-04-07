// Tests for Problem 1963: Minimum Number of Swaps to Make the String Balanced
// Java reference: src/test/java/g1901_2000/s1963_minimum_number_of_swaps_to_make_the_string_balanced/SolutionTest.java

use leetcode_in_rust::s1963::minimum_number_of_swaps_to_make_the_string_balanced::Solution;

#[test]
fn test_min_swaps() {
    assert_eq!(Solution::min_swaps("][][".to_string()), 1);
}

#[test]
fn test_min_swaps2() {
    assert_eq!(Solution::min_swaps("]]][[[".to_string()), 2);
}

#[test]
fn test_min_swaps3() {
    assert_eq!(Solution::min_swaps("[]".to_string()), 0);
}
