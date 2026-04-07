// Tests for Problem 1864: Minimum Number of Swaps to Make the Binary String Alternating
// Java reference: src/test/java/g1801_1900/s1864_minimum_number_of_swaps_to_make_the_binary_string_alternating/SolutionTest.java

use leetcode_in_rust::s1864::minimum_number_of_swaps_to_make_the_binary_string_alternating::Solution;

#[test]
fn test_min_swaps() {
    assert_eq!(Solution::min_swaps("111000".to_string()), 1);
}

#[test]
fn test_min_swaps2() {
    assert_eq!(Solution::min_swaps("010".to_string()), 0);
}

#[test]
fn test_min_swaps3() {
    assert_eq!(Solution::min_swaps("1110".to_string()), -1);
}
