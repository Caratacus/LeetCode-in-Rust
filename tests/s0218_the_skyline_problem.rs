// Tests for Problem 0218: The Skyline Problem
// Java reference: src/test/java/g0201_0300/s0218_the_skyline_problem/SolutionTest.java

use leetcode_in_rust::s0218::the_skyline_problem::Solution;

#[test]
fn test_get_skyline() {
    let buildings = vec![vec![2, 9, 10], vec![3, 7, 15], vec![5, 12, 12], vec![15, 20, 10], vec![19, 24, 8]];
    let expected = vec![vec![2, 10], vec![3, 15], vec![7, 12], vec![12, 0], vec![15, 10], vec![20, 8], vec![24, 0]];
    assert_eq!(Solution::get_skyline(buildings), expected);
}

#[test]
fn test_get_skyline2() {
    let buildings = vec![vec![0, 2, 3], vec![2, 5, 3]];
    let expected = vec![vec![0, 3], vec![5, 0]];
    assert_eq!(Solution::get_skyline(buildings), expected);
}
