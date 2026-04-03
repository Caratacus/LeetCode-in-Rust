// Tests for Problem 0546: Remove Boxes
// Java reference: src/test/java/g0501_0600/s0546_remove_boxes/SolutionTest.java

use leetcode_in_rust::s0546::remove_boxes::Solution;

#[test]
fn test_remove_boxes() {
    assert_eq!(
        Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]),
        23
    );
}

#[test]
fn test_remove_boxes2() {
    assert_eq!(Solution::remove_boxes(vec![1, 1, 1]), 9);
}

#[test]
fn test_remove_boxes3() {
    assert_eq!(Solution::remove_boxes(vec![1]), 1);
}
