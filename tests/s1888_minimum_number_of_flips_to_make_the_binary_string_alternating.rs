// Tests for Problem 1888: Minimum Number of Flips to Make the Binary String Alternating
// Java reference: src/test/java/g1801_1900/s1888_minimum_number_of_flips_to_make_the_binary_string_alternating/SolutionTest.java

use leetcode_in_rust::s1888::minimum_number_of_flips_to_make_the_binary_string_alternating::Solution;

#[test]
fn test_min_flips() {
    assert_eq!(Solution::min_flips("111000".to_string()), 2);
}

#[test]
fn test_min_flips2() {
    assert_eq!(Solution::min_flips("010".to_string()), 0);
}

#[test]
fn test_min_flips3() {
    assert_eq!(Solution::min_flips("1110".to_string()), 1);
}
