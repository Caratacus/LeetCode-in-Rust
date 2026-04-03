// Tests for Problem 0583: Delete Operation for Two Strings
// Java reference: src/test/java/g0501_0600/s0583_delete_operation_for_two_strings/SolutionTest.java

use leetcode_in_rust::s0583::delete_operation_for_two_strings::Solution;

#[test]
fn test_min_distance() {
    assert_eq!(Solution::min_distance("sea".to_string(), "eat".to_string()), 2);
}

#[test]
fn test_min_distance2() {
    assert_eq!(Solution::min_distance("leetcode".to_string(), "etco".to_string()), 4);
}
