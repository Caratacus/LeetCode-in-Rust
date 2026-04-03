// Tests for Problem 1790: Check if One String Swap Can Make Strings Equal
// Java reference: src/test/java/g1701_1800/s1790_check_if_one_string_swap_can_make_strings_equal/SolutionTest.java

use leetcode_in_rust::s1790::check_if_one_string_swap_can_make_strings_equal::Solution;

#[test]
fn test_are_almost_equal() {
    assert_eq!(Solution::are_almost_equal("bank".to_string(), "kanb".to_string()), true);
}

#[test]
fn test_are_almost_equal2() {
    assert_eq!(Solution::are_almost_equal("attack".to_string(), "defend".to_string()), false);
}

#[test]
fn test_are_almost_equal3() {
    assert_eq!(Solution::are_almost_equal("kelb".to_string(), "kelb".to_string()), true);
}
