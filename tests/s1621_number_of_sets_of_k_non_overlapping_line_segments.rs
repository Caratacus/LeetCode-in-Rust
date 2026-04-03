// Tests for Problem 1621: Number of Sets of K Non-Overlapping Line Segments
// Java reference: src/test/java/g1601_1700/s1621_number_of_sets_of_k_non_overlapping_line_segments/SolutionTest.java

use leetcode_in_rust::s1621::number_of_sets_of_k_non_overlapping_line_segments::Solution;

#[test]
fn test_number_of_sets() {
    assert_eq!(Solution::number_of_sets(4, 2), 5);
}

#[test]
fn test_number_of_sets2() {
    assert_eq!(Solution::number_of_sets(3, 1), 3);
}

#[test]
fn test_number_of_sets3() {
    assert_eq!(Solution::number_of_sets(30, 7), 796297179);
}
