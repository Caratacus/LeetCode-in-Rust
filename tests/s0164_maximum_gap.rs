// Tests for Problem 0164: Maximum Gap
// Java reference: src/test/java/g0121_0200/s0164_maximum_gap/SolutionTest.java

use leetcode_in_rust::s0164::maximum_gap::Solution;

#[test]
fn test_maximum_gap() {
    assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
}

#[test]
fn test_maximum_gap2() {
    assert_eq!(Solution::maximum_gap(vec![10]), 0);
}
