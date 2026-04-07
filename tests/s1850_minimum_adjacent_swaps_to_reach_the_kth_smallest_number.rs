// Tests for Problem 1850: Minimum Adjacent Swaps to Reach the Kth Smallest Number
// Java reference: src/test/java/g1801_1900/s1850_minimum_adjacent_swaps_to_reach_the_kth_smallest_number/SolutionTest.java

use leetcode_in_rust::s1850::minimum_adjacent_swaps_to_reach_the_kth_smallest_number::Solution;

#[test]
fn test_get_min_swaps() {
    assert_eq!(Solution::get_min_swaps("5489355142".to_string(), 4), 2);
}

#[test]
fn test_get_min_swaps2() {
    assert_eq!(Solution::get_min_swaps("11112".to_string(), 4), 4);
}

#[test]
fn test_get_min_swaps3() {
    assert_eq!(Solution::get_min_swaps("00123".to_string(), 1), 1);
}
