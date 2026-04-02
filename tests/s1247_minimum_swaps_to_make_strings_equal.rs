// Tests for Problem 1247: Minimum Swaps to Make Strings Equal
// Java reference: src/test/java/g1201_1300/s1247_minimum_swaps_to_make_strings_equal/SolutionTest.java

use leetcode_in_rust::s1247::minimum_swaps_to_make_strings_equal::Solution;

#[test]
fn test_minimum_swap() {
    assert_eq!(Solution::minimum_swap("xx".to_string(), "yy".to_string()), 1);
}

#[test]
fn test_minimum_swap2() {
    assert_eq!(Solution::minimum_swap("xy".to_string(), "yx".to_string()), 2);
}

#[test]
fn test_minimum_swap3() {
    assert_eq!(Solution::minimum_swap("xx".to_string(), "xy".to_string()), -1);
}
