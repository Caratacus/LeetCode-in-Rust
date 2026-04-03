// Tests for Problem 1758: Minimum Changes To Make Alternating Binary String
// Java reference: src/test/java/g1701_1800/s1758_minimum_changes_to_make_alternating_binary_string/SolutionTest.java

use leetcode_in_rust::s1758::minimum_changes_to_make_alternating_binary_string::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations("0100".to_string()), 1);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations("10".to_string()), 0);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations("1111".to_string()), 2);
}
