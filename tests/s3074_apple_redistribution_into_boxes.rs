// Tests for Problem 3074: Apple Redistribution into Boxes
// Java reference: src/test/java/g3001_3100/s3074_apple_redistribution_into_boxes/SolutionTest.java

use leetcode_in_rust::s3074::apple_redistribution_into_boxes::Solution;

#[test]
fn test_minimum_boxes() {
    assert_eq!(
        Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]),
        2
    );
}

#[test]
fn test_minimum_boxes2() {
    assert_eq!(
        Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]),
        4
    );
}
