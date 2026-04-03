// Tests for Problem 1576: Replace All ?'s to Avoid Consecutive Repeating Characters
// Java reference: src/test/java/g1501_1600/s1576_replace_all_s_to_avoid_consecutive_repeating_characters/SolutionTest.java

use leetcode_in_rust::s1576::replace_all_s_to_avoid_consecutive_repeating_characters::Solution;

#[test]
fn test_modify_string() {
    assert_eq!(Solution::modify_string("?zs".to_string()), "azs");
}

#[test]
fn test_modify_string2() {
    assert_eq!(Solution::modify_string("ubv?w".to_string()), "ubvaw");
}
