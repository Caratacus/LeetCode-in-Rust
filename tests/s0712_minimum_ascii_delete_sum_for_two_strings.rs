// Tests for Problem 0712: Minimum ASCII Delete Sum for Two Strings
// Java reference: src/test/java/g0701_0800/s0712_minimum_ascii_delete_sum_for_two_strings/SolutionTest.java

use leetcode_in_rust::s0712::minimum_ascii_delete_sum_for_two_strings::Solution;

#[test]
fn test_minimum_delete_sum() {
    assert_eq!(Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()), 231);
}

#[test]
fn test_minimum_delete_sum2() {
    assert_eq!(Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()), 403);
}
