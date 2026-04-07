// Tests for Problem 1784: Check if Binary String Has at Most One Segment of Ones
// Java reference: src/test/java/g1701_1800/s1784_check_if_binary_string_has_at_most_one_segment_of_ones/SolutionTest.java

use leetcode_in_rust::s1784::check_if_binary_string_has_at_most_one_segment_of_ones::Solution;

#[test]
fn test_check_ones_segment() {
    assert_eq!(Solution::check_ones_segment("1001".to_string()), false);
}

#[test]
fn test_check_ones_segment2() {
    assert_eq!(Solution::check_ones_segment("110".to_string()), true);
}
