// Tests for Problem 0084: Largest Rectangle in Histogram
// Java reference: src/test/java/g0001_0100/s0084_largest_rectangle_in_histogram/SolutionTest.java

use leetcode_in_rust::s0084::largest_rectangle_in_histogram::Solution;

#[test]
fn test_largest_rectangle_area() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
}

#[test]
fn test_largest_rectangle_area2() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
}

#[test]
fn test_largest_rectangle_area3() {
    assert_eq!(Solution::largest_rectangle_area(vec![]), 0);
}
