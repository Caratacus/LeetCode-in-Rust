// Tests for Problem 1037: Valid Boomerang
// Java reference: src/test/java/g1001_1100/s1037_valid_boomerang/SolutionTest.java

use leetcode_in_rust::s1037::valid_boomerang::Solution;

#[test]
fn test_is_boomerang() {
    assert_eq!(Solution::is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]), true);
}

#[test]
fn test_is_boomerang2() {
    assert_eq!(Solution::is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), false);
}
