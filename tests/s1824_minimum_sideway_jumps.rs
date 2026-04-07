// Tests for Problem 1824: Minimum Sideway Jumps
// Java reference: src/test/java/g1801_1900/s1824_minimum_sideway_jumps/SolutionTest.java

use leetcode_in_rust::s1824::minimum_sideway_jumps::Solution;

#[test]
fn test_min_side_jumps() {
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
}

#[test]
fn test_min_side_jumps2() {
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
}

#[test]
fn test_min_side_jumps3() {
    assert_eq!(Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]), 2);
}
