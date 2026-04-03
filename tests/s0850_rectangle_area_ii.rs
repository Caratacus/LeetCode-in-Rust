// Tests for Problem 0850: Rectangle Area II
// Java reference: src/test/java/g0801_0900/s0850_rectangle_area_ii/SolutionTest.java

use leetcode_in_rust::s0850::rectangle_area_ii::Solution;

#[test]
fn test_rectangle_area() {
    assert_eq!(
        Solution::rectangle_area(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]]),
        6
    );
}

#[test]
fn test_rectangle_area2() {
    assert_eq!(
        Solution::rectangle_area(vec![vec![0, 0, 1000000000, 1000000000]]),
        49
    );
}
