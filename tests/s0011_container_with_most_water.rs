// Tests for Problem 0011: Container With Most Water
// Java reference: src/test/java/g0001_0100/s0011_container_with_most_water/SolutionTest.java

use leetcode_in_rust::s0011::container_with_most_water::Solution;

#[test]
fn test_max_area() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}

#[test]
fn test_max_area2() {
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
