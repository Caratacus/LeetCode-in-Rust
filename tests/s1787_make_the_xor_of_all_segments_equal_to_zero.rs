// Tests for Problem 1787: Make the XOR of All Segments Equal to Zero
// Java reference: src/test/java/g1701_1800/s1787_make_the_xor_of_all_segments_equal_to_zero/SolutionTest.java

use leetcode_in_rust::s1787::make_the_xor_of_all_segments_equal_to_zero::Solution;

#[test]
fn test_min_changes() {
    assert_eq!(Solution::min_changes(vec![1, 2, 0, 3, 0], 1), 3);
}

#[test]
fn test_min_changes2() {
    assert_eq!(
        Solution::min_changes(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3),
        3
    );
}

#[test]
fn test_min_changes3() {
    assert_eq!(
        Solution::min_changes(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3),
        3
    );
}
