// Tests for Problem 2914: Minimum Number of Changes to Make Binary String Beautiful
// Java reference: src/test/java/g2901_3000/s2914_minimum_number_of_changes_to_make_binary_string_beautiful/SolutionTest.java

use leetcode_in_rust::s2914::minimum_number_of_changes_to_make_binary_string_beautiful::Solution;

#[test]
fn test_min_changes() {
    assert_eq!(Solution::min_changes("1001".to_string()), 2);
}

#[test]
fn test_min_changes2() {
    assert_eq!(Solution::min_changes("10".to_string()), 1);
}

#[test]
fn test_min_changes3() {
    assert_eq!(Solution::min_changes("0000".to_string()), 0);
}
