// Tests for Problem 1739: Building Boxes
// Java reference: src/test/java/g1701_1800/s1739_building_boxes/SolutionTest.java

use leetcode_in_rust::s1739::building_boxes::Solution;

#[test]
fn test_minimum_boxes() {
    assert_eq!(Solution::minimum_boxes(3), 3);
}

#[test]
fn test_minimum_boxes2() {
    assert_eq!(Solution::minimum_boxes(4), 3);
}

#[test]
fn test_minimum_boxes3() {
    assert_eq!(Solution::minimum_boxes(10), 6);
}
