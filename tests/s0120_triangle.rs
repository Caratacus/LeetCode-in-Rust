// Tests for Problem 0120: Triangle
// Java reference: src/test/java/g0101_0200/s0120_triangle/SolutionTest.java

use leetcode_in_rust::s0120::triangle::Solution;

#[test]
fn test_minimum_total() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    assert_eq!(Solution::minimum_total(triangle), 11);
}

#[test]
fn test_minimum_total2() {
    let triangle = vec![vec![-10]];
    assert_eq!(Solution::minimum_total(triangle), -10);
}
